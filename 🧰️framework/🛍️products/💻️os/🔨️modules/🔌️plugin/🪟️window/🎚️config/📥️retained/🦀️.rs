//! 📥️ Retained typed Pack and SPR loading for one exact window-config partition.

use super::{ WindowConfigOwner, WindowConfigPack, WindowConfigPartition, WindowRegistry};
use crate::{protocol, store};
use std::any::Any;
use std::collections::HashMap;
use std::mem::ManuallyDrop;
use semio_framework_value::{ValueError, ValueRefusalKind, RetirementDemand, retirement::{RetireOwned, RetirementCursor, controlled::ControlledRetirement}, retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}};
use super::PluginLifecycleStep;

type Mounted = store::mounted_pack_rt::RetainedValueToken;

#[cfg(test)]
mod physical_ownership_tests {
    use super::*;
    use super::super::pack_identity_tests::IdentityWindowOwner;

    #[test]
    fn window_config_paged_registry_actual_authored_root_metadata_producer() {
        use super::super::retained_pack_load_tests::RetainedLoadCameraConfig;
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../🗂️registry/🧫️fixtures/🔣️.json")).unwrap();
        let law = &fixture["rootMetadata"];
        let ((producer, absent), getter) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| (
            <RetainedLoadCameraConfig as store::ArtifactPack>::record_spec_producer(),
            <semio_framework_value::DslValue as store::ArtifactPack>::record_spec_producer(),
        ));
        assert_eq!(producer.is_some(), law["ownedProducer"].as_bool().unwrap());
        assert_eq!(absent.is_some(), law["schemaLessProducer"].as_bool().unwrap());
        assert_eq!((getter.requested_bytes, getter.released_bytes), (law["getterAllocationBytes"].as_u64().unwrap() as usize, law["getterReleaseBytes"].as_u64().unwrap() as usize));
        assert_eq!(<RetainedLoadCameraConfig as store::ArtifactDsl>::envelope_id(), law["owner"].as_str().unwrap());
        let mut accepted = |_| true;
        let mut native = semio_framework_value::NativeDecodeControl::new_retained(&mut accepted);
        native.admit_turn_capacity(law["maximumCapacityBytes"].as_u64().unwrap() as usize).unwrap();
        let (spec, birth) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| producer.unwrap().decode(&mut native).unwrap());
        assert_eq!(birth.requested_bytes, native.owned_bytes());
        assert_eq!(birth.released_bytes, 0);
        assert_eq!(spec.fields.iter().map(|field| field.key.as_str()).collect::<Vec<_>>(), law["fields"].as_array().unwrap().iter().map(|field| field.as_str().unwrap()).collect::<Vec<_>>());
        let mut owner = ControlledRetirement::new(spec).map_err(|(error, _)| error).unwrap();
        let mut allocated = birth.requested_bytes;
        let mut released = 0;
        let body = fixture["maximumPageBytes"].as_u64().unwrap() as usize;
        while !owner.terminal_is_empty() {
            let demand = controlled_demands(&owner, body).unwrap();
            let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes.max(body), maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth };
            let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.step(grant));
            let progress = result.unwrap().progress();
            assert!(progress.fits(grant));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (progress.retained_capacity_bytes, progress.released_bytes));
            allocated += heap.requested_bytes;
            released += heap.released_bytes;
        }
        assert_eq!(allocated, released);
        let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| drop(owner));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        eprintln!("[DEBUG] original Window authored root metadata nativeBirth={} controlledRelease={released} optionalGetter0 terminalDrop0", birth.requested_bytes);
    }

    #[test]
    fn window_config_paged_registry_retained_source_reservation_uses_capacity_authority() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../🗂️registry/🧫️fixtures/🔣️.json")).unwrap();
        let body = fixture["maximumPageBytes"].as_u64().unwrap() as usize;
        let mut decoder = RetainedWindowConfigStateDecode::<IdentityWindowOwner>::new();
        decoder.phase = RetainedStatePhase::Ingress;
        *decoder.source = Some(store::mounted_pack_rt::RetainedPackSourceCursor::try_new(1, 1, 65_536).unwrap());
        let capacity = decoder.source.as_ref().unwrap().next_allocation_bytes().unwrap();
        let refused_law = &fixture["sourceReservation"][0];
        let admitted_law = &fixture["sourceReservation"][1];
        let refused = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: refused_law["copyBytes"].as_u64().unwrap() as usize, maximum_depth: 64, ..Default::default() };
        let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| decoder.advance(&[7], refused));
        assert_eq!(result.unwrap(), refused_law["reserved"].as_bool().unwrap());
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        assert!(!decoder.source.as_ref().unwrap().has_reserved_page());
        let admitted = RetainedCloneGrant { maximum_copy_bytes: admitted_law["copyBytes"].as_u64().unwrap() as usize, maximum_capacity_bytes: capacity, ..refused };
        let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| decoder.advance(&[7], admitted));
        assert_eq!(result.unwrap(), admitted_law["reserved"].as_bool().unwrap());
        assert_eq!((heap.requested_bytes, heap.released_bytes), (capacity, 0));
        assert_eq!(decoder.admitted, admitted_law["copiedBytes"].as_u64().unwrap() as usize);
        let mut released = 0;
        while !decoder.terminal_is_empty() {
            let demand = decoder.retirement_demands(body).unwrap();
            let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes.max(body), maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth };
            let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| decoder.close_step(grant));
            let progress = result.unwrap().progress();
            assert!(progress.fits(grant));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (progress.retained_capacity_bytes, progress.released_bytes));
            released += heap.released_bytes;
        }
        assert_eq!(released, capacity);
        let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| drop(decoder));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        eprintln!("[DEBUG] original retained window source capacity={capacity} release={released} copy0 reservation admitted terminalDrop0");
    }

    #[test]
    fn window_config_paged_registry_actual_inline_parser_retains_original_backing() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../🗂️registry/🧫️fixtures/🔣️.json")).unwrap();
        let body = fixture["maximumPageBytes"].as_u64().unwrap() as usize;
        let mut accepted = |_| true;
        let mut native = semio_framework_value::NativeDecodeControl::new_retained(&mut accepted);
        let (mut parser, birth) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| {
            let mut parser = RetainedWindowConfigTypedState::<IdentityWindowOwner>::new(&mut native).unwrap();
            let original_fields = parser.spec.fields.as_ptr();
            parser.begin_container(store::mounted_pack_rt::RetainedValueContainer::Record, 0).unwrap();
            assert_eq!(parser.spec.fields.as_ptr(), original_fields);
            assert!(matches!(parser.stack.last(), Some(ValueFrame::Record { root: true, spec: None, .. })));
            parser
        });
        let mut allocated = birth.requested_bytes;
        let mut released = birth.released_bytes;
        let mut turns = 0;
        while !parser.terminal_is_empty() {
            let demand = parser.retirement_demands(body).unwrap();
            let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes.max(body), maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth };
            let mut refused = vec![RetainedCloneGrant { maximum_items: 0, ..grant }];
            if demand.copy_bytes > 0 { refused.push(RetainedCloneGrant { maximum_copy_bytes: demand.copy_bytes - 1, ..grant }); }
            if demand.capacity_bytes > 0 { refused.push(RetainedCloneGrant { maximum_capacity_bytes: demand.capacity_bytes - 1, ..grant }); }
            if demand.release_bytes > 0 { refused.push(RetainedCloneGrant { maximum_release_bytes: demand.release_bytes - 1, ..grant }); }
            if demand.depth > 0 { refused.push(RetainedCloneGrant { maximum_depth: demand.depth - 1, ..grant }); }
            for refused in refused {
                let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| parser.close_step(refused));
                assert_eq!(result.unwrap().progress(), RetainedCloneProgress::default());
                assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            }
            let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| parser.close_step(grant));
            let progress = result.unwrap().progress();
            assert!(progress.fits(grant));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (progress.retained_capacity_bytes, progress.released_bytes));
            allocated += heap.requested_bytes;
            released += heap.released_bytes;
            turns += 1;
            assert!(turns < 65_536);
        }
        assert_eq!(allocated, released);
        let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| drop(parser));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        eprintln!("[DEBUG] original inline window parser allocated={allocated} release={released} turns={turns} terminalDrop0");
    }
}

fn controlled_demands<T: RetireOwned>(owner: &ControlledRetirement<T>, body: usize) -> Result<RetirementDemand, ValueError> {
    Ok(RetirementDemand { copy_bytes: owner.next_copy_byte_demand()?, capacity_bytes: owner.next_capacity_byte_demand(body)?, release_bytes: owner.next_release_byte_demand()?, depth: owner.next_depth_demand()? })
}
fn pack_close_receipt(step: store::mounted_pack_rt::RetainedPackCloseStep, demand: RetirementDemand) -> RetainedCloneStep {
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
    Ready,
    Rejected(WindowConfigPackLoadDiagnostic),
    Complete,
}

pub(super) trait ErasedWindowConfigPackLoad: Send {
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn window_kind_id(&self) -> &str;
    fn registry_lifetime(&self) -> u64;
    fn phase(&self) -> WindowConfigPackLoadPhase;
    fn progress(&self) -> WindowConfigPackLoadProgress;
    fn diagnostic(&self) -> Option<WindowConfigPackLoadDiagnostic>;
    fn advance(&mut self, grant: RetainedCloneGrant) -> WindowConfigPackLoadStep;
    fn request_cancel(&mut self);
    fn reject_stale(&mut self) -> WindowConfigPackLoadStep;
    fn retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError>;
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, String>;
    fn terminal_is_empty(&self) -> bool;
}

pub struct WindowConfigPackLoad {
    pub(super) inner:ManuallyDrop<Option<Box<dyn ErasedWindowConfigPackLoad>>>,
    closed_progress:WindowConfigPackLoadProgress,
    closed_diagnostic:Option<WindowConfigPackLoadDiagnostic>,
}
impl WindowConfigPackLoad {
    #[cfg(test)]
    pub(super) fn candidate_actor_for_test<O:WindowConfigOwner>(&mut self)->Option<protocol::ActorId>{let typed=self.inner.as_mut()?.as_any_mut().downcast_mut::<TypedWindowConfigPackLoad<O>>()?;typed.candidate.as_ref().map(|candidate|candidate.store.local_actor_id().clone())}
    pub fn phase(&self)->WindowConfigPackLoadPhase{self.inner.as_ref().map_or(WindowConfigPackLoadPhase::Complete,|inner|inner.phase())}
    pub fn progress(&self)->WindowConfigPackLoadProgress{self.inner.as_ref().map_or(self.closed_progress,|inner|inner.progress())}
    pub fn diagnostic(&self)->Option<WindowConfigPackLoadDiagnostic>{self.inner.as_ref().map_or(self.closed_diagnostic,|inner|inner.diagnostic())}
    pub fn request_cancel(&mut self){if let Some(inner)=self.inner.as_mut(){inner.request_cancel();}}
    pub fn cold_work_grant(&mut self)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES.max(self.inner.as_mut().map_or(0,|inner|inner.demand_bytes())),maximum_capacity_bytes:store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES,maximum_release_bytes:store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES,maximum_depth:64}}
    pub fn terminal_is_empty(&self)->bool{self.inner.is_none()}
    pub fn retirement_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
        let Some(inner)=self.inner.as_ref()else{return Ok(Default::default());};
        if inner.terminal_is_empty(){return Ok(RetirementDemand{release_bytes:std::mem::size_of_val(inner.as_ref()),depth:1,..Default::default()});}
        let mut demand=inner.close_demands(body)?;demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"window load frame depth overflow"))?;Ok(demand)
    }

    pub fn phase(&self) -> WindowConfigPackLoadPhase {
        self.inner.phase()
    }

    pub fn progress(&self) -> WindowConfigPackLoadProgress {
        self.inner.progress()
    }

    pub fn diagnostic(&self) -> Option<WindowConfigPackLoadDiagnostic> {
        self.inner.diagnostic()
    }

    pub fn request_cancel(&mut self) {
        self.inner.request_cancel();
    }

    /// 🎟️ Fixed caller policy; each currency retains its independent authority.
    pub const fn next_grant(&self) -> RetainedCloneGrant {
        RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 65_536, maximum_capacity_bytes: 65_536, maximum_release_bytes: 65_536, maximum_depth: 64 }
    }

    pub fn retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> { self.inner.retirement_demands(body) }

    pub fn terminal_is_empty(&self) -> bool {
        self.inner.terminal_is_empty()
    }
}

impl Drop for WindowConfigPackLoad {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.inner.terminal_is_empty(), "window config Pack load reached Drop before terminal-empty handoff or retirement");
    }
}

#[derive(Clone, Copy)]
struct ExpectedValue {
    shape: Option<semio_framework_dsl_record::BorrowedShape>,
    dsl: bool,
}

impl ExpectedValue {
    fn field(shape: Option<semio_framework_dsl_record::BorrowedShape>) -> Self {
        Self { shape, dsl: false }
    }

    fn dsl() -> Self {
        Self { shape: None, dsl: true }
    }
}

#[derive(semio_framework_value::RetireOwned)]
enum BuiltValue {
    Field(semio_framework_dsl_record::FieldValue),
    Dsl(semio_framework_value::DslValue),
}

#[derive(semio_framework_value::RetireOwned)]
enum ValueFrame {
    Record { kind: store::mounted_pack_rt::RetainedValueContainer, root: bool, spec: Option<semio_framework_dsl_record::BorrowedRecordSpec>, fields: semio_framework_dsl_record::RecordFields, field: Option<u16> },
    Sequence { kind: store::mounted_pack_rt::RetainedValueContainer, element: ExpectedValue, values: Vec<BuiltValue> },
    Map { dsl: bool, element: ExpectedValue, values: Vec<(String, BuiltValue)>, key: Option<String> },
    Statements { variants: &'static [(&'static str, fn() -> semio_framework_dsl_record::BorrowedRecordSpec)], values: Vec<(String, semio_framework_dsl_record::RecordValue)>, keyword: Option<String> },
    Bytes { values: Vec<u8>, remaining: usize },
}

#[derive(Clone, Copy, semio_framework_value::RetireOwned)]
enum ValueWrapper {
    Block,
    Dynamic,
}

#[derive(semio_framework_value::RetireOwned)]
enum StringTarget {
    Value(ExpectedValue),
    MapKey,
    StatementKeyword,
}

#[derive(semio_framework_value::RetireOwned)]
struct RetainedString {
    target: StringTarget,
    value: String,
    remaining: Option<u64>,
    symbol: Option<(u64, usize, usize)>,
}

struct RetainedWindowConfigTypedState<O: WindowConfigOwner> {
    spec: semio_framework_dsl_record::BorrowedRecordSpec,
    stack: Vec<ValueFrame>,
    wrappers: Vec<(ValueWrapper, usize)>,
    string: Option<RetainedString>,
    tag: Option<u8>,
    root: Option<O::State>,
    complete: bool,
    handed_back: bool,
    retirement_original: ManuallyDrop<Option<RetainedTypedOwners<O>>>,
    retirement: Option<ControlledRetirement<RetainedTypedOwners<O>>>,
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
    fn retirement_element_copy_bytes() -> usize { std::mem::size_of::<Self>() }
}

#[derive(semio_framework_value::RetireOwned)]
struct RetainedTypedOwners<O: WindowConfigOwner> {
    spec: semio_framework_dsl_record::BorrowedRecordSpec,
    stack: Vec<ValueFrame>,
    wrappers: Vec<(ValueWrapper, usize)>,
    string: Option<RetainedString>,
    root: Option<O::State>,
}

impl<O: WindowConfigOwner> RetainedWindowConfigTypedState<O> {
    fn new(native: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, WindowConfigPackLoadDiagnostic> {
        let producer = <O::State as store::ArtifactPack>::borrowed_record_spec_producer().ok_or(WindowConfigPackLoadDiagnostic::TypedState)?;
        let spec = producer.decode(native).map_err(|_| WindowConfigPackLoadDiagnostic::TypedState)?;
        let stack = Vec::new();
        let wrappers = Vec::new();
        Ok(Self { spec, stack, wrappers, string: None, tag: None, root: None, complete: false, handed_back: false, retirement_original: ManuallyDrop::new(None), retirement: None })
    }

    /// 🎁️ Wrappers pending at the current container depth, outermost first; deeper values never see them.
    fn pending_wrappers(&self) -> &[(ValueWrapper, usize)] {
        let depth = self.stack.len();
        &self.wrappers[self.wrappers.iter().rposition(|(_, at)| *at != depth).map_or(0, |index| index + 1)..]
    }

    fn expected(&self) -> Result<ExpectedValue, WindowConfigPackLoadDiagnostic> {
        self.pending_wrappers().iter().try_fold(self.parent_expected()?, |expected, (wrapper, _)| {
            Ok(match wrapper {
                ValueWrapper::Block => match expected.shape {
                    Some(semio_framework_dsl_record::BorrowedShape::Block(inner)) => ExpectedValue::field(Some(inner())),
                    _ => ExpectedValue::field(None),
                },
                ValueWrapper::Dynamic => ExpectedValue::dsl(),
            })
        })
    }

    fn parent_expected(&self) -> Result<ExpectedValue, WindowConfigPackLoadDiagnostic> {
        match self.stack.last() {
            None if self.root.is_none() => Ok(ExpectedValue::field(None)),
            Some(ValueFrame::Record { root, spec, field: Some(field), .. }) => Ok(ExpectedValue::field((if *root { Some(&self.spec) } else { spec.as_ref() }).and_then(|spec| spec.fields.iter().find(|candidate| candidate.id == *field)).map(|field| field.shape))),
            Some(ValueFrame::Sequence { element, .. }) => Ok(*element),
            Some(ValueFrame::Map { element, key: Some(_), .. }) => Ok(*element),
            Some(ValueFrame::Statements { variants, keyword: Some(keyword), .. }) => Ok(ExpectedValue::field(variants.iter().find(|(candidate, _)| *candidate == keyword).map(|(_, spec)| semio_framework_dsl_record::BorrowedShape::Record(*spec)))),
            _ => Err(WindowConfigPackLoadDiagnostic::TypedState),
        }
    }

    fn child_record_spec(expected: &ExpectedValue) -> Option<semio_framework_dsl_record::BorrowedRecordSpec> {
        match expected.shape.as_ref() {
            Some(semio_framework_dsl_record::BorrowedShape::Record(spec)) | Some(semio_framework_dsl_record::BorrowedShape::Table(spec)) => Some(spec()),
            _ => None,
        }
    }

    /// 🎯️ The DSL shapes whose VALUE is a [`semio_framework_dsl_record::FieldValue::Tuple`] even though the
    /// pack's packed-numeric wire form (`TAG_PACKED_F64`/`TAG_PACKED_VARINT`, `🎒️pack/🌱️value/🦀️.rs`
    /// `encode_seq`) carries no tuple-vs-list marker of its own: a coordinate (`@x,y,z`), a direction,
    /// a dimension triple and a range are each a small tuple of floats. The whole-pack decoder spells
    /// exactly this set in its own `is_tuple_shape`; this RETAINED decoder knew only `Shape::Tuple`, so
    /// a `#[dsl(coord)] [f64; 3]` field was rebuilt as a `FieldValue::List` and the state's
    /// `DslField::from_value` rejected it — `WindowConfigPackLoadDiagnostic::TypedState`, surfaced as
    /// `window-config.typed-state`. Every CAD world window config reload died on `CadCamera::position`
    /// (ticket 26/09/19/SEMIO-TECH-PLAY-GRID-WITH-EVERY-APP, cad-content).
    fn shape_is_tuple_valued(shape: Option<&semio_framework_dsl_record::BorrowedShape>) -> bool {
        matches!(
            shape,
            Some(semio_framework_dsl_record::BorrowedShape::Tuple(_, _)) | Some(semio_framework_dsl_record::BorrowedShape::Coord(_)) | Some(semio_framework_dsl_record::BorrowedShape::Dir) | Some(semio_framework_dsl_record::BorrowedShape::Dim(_)) | Some(semio_framework_dsl_record::BorrowedShape::Range)
        )
    }

    /// 📍️ Coordinate/direction/dimension/range literals are fixed-arity tuples of floats; their
    /// elements carry no element shape of their own, so name it here rather than leave every
    /// component shapeless.
    fn child_element(expected: &ExpectedValue) -> ExpectedValue {
        let shape = match expected.shape.as_ref() {
            Some(semio_framework_dsl_record::BorrowedShape::Tuple(inner, _)) | Some(semio_framework_dsl_record::BorrowedShape::List(inner)) | Some(semio_framework_dsl_record::BorrowedShape::Map(inner)) => Some(inner()),
            Some(semio_framework_dsl_record::BorrowedShape::Table(spec)) => Some(semio_framework_dsl_record::BorrowedShape::Record(*spec)),
            Some(semio_framework_dsl_record::BorrowedShape::Coord(_)) | Some(semio_framework_dsl_record::BorrowedShape::Dir) | Some(semio_framework_dsl_record::BorrowedShape::Dim(_)) | Some(semio_framework_dsl_record::BorrowedShape::Range) => Some(semio_framework_dsl_record::BorrowedShape::Float),
            _ => None,
        };
        ExpectedValue { shape, dsl: expected.dsl }
    }

    fn string_target(&self) -> Result<StringTarget, WindowConfigPackLoadDiagnostic> {
        match self.stack.last() {
            Some(ValueFrame::Map { key: None, .. }) => Ok(StringTarget::MapKey),
            Some(ValueFrame::Statements { keyword: None, .. }) => Ok(StringTarget::StatementKeyword),
            _ => Ok(StringTarget::Value(self.expected()?)),
        }
    }

    fn begin_string(&mut self) -> Result<(), WindowConfigPackLoadDiagnostic> {
        if self.string.is_some() {
            return Err(WindowConfigPackLoadDiagnostic::TypedState);
        }
        self.string = Some(RetainedString { target: self.string_target()?, value: String::new(), remaining: None, symbol: None });
        Ok(())
    }

    fn begin_symbol(&mut self, symbol: u64, catalog: &store::mounted_pack_rt::RetainedPackCatalogCursor) -> Result<(), WindowConfigPackLoadDiagnostic> {
        self.begin_string()?;
        let chars = catalog.symbol_chars(symbol).map_err(|_| WindowConfigPackLoadDiagnostic::Pack)?;
        let owner = self.string.as_mut().expect("retained config string was just created");
        owner.value.try_reserve_exact(chars.saturating_mul(4)).map_err(|_| WindowConfigPackLoadDiagnostic::Capacity)?;
        owner.symbol = Some((symbol, 0, chars));
        if chars == 0 {
            self.finish_string()?;
        }
        Ok(())
    }

    fn grant_symbol(&mut self, catalog: &store::mounted_pack_rt::RetainedPackCatalogCursor) -> Result<bool, WindowConfigPackLoadDiagnostic> {
        let Some(owner) = self.string.as_mut() else { return Ok(false) };
        let Some((symbol, index, chars)) = owner.symbol else { return Ok(false) };
        let value = catalog.symbol_char(symbol, index).map_err(|_| WindowConfigPackLoadDiagnostic::Pack)?.ok_or(WindowConfigPackLoadDiagnostic::Pack)?;
        owner.value.push(value);
        if index + 1 == chars {
            self.finish_string()?;
        } else {
            self.string.as_mut().expect("retained config symbol remains owned").symbol = Some((symbol, index + 1, chars));
        }
        Ok(true)
    }

    fn finish_string(&mut self) -> Result<(), WindowConfigPackLoadDiagnostic> {
        let owner = self.string.take().ok_or(WindowConfigPackLoadDiagnostic::TypedState)?;
        match owner.target {
            StringTarget::MapKey => match self.stack.last_mut() {
                Some(ValueFrame::Map { key, .. }) if key.is_none() => *key = Some(owner.value),
                _ => return Err(WindowConfigPackLoadDiagnostic::TypedState),
            },
            StringTarget::StatementKeyword => match self.stack.last_mut() {
                Some(ValueFrame::Statements { keyword, .. }) if keyword.is_none() => *keyword = Some(owner.value),
                _ => return Err(WindowConfigPackLoadDiagnostic::TypedState),
            },
            StringTarget::Value(expected) => {
                let value = if expected.dsl { BuiltValue::Dsl(semio_framework_value::DslValue::String(owner.value)) } else { BuiltValue::Field(semio_framework_dsl_record::FieldValue::Text(owner.value)) };
                self.emit(value)?;
            }
        }
        Ok(())
    }

    fn into_dsl(value: BuiltValue) -> Result<semio_framework_value::DslValue, WindowConfigPackLoadDiagnostic> {
        match value {
            BuiltValue::Dsl(value) => Ok(value),
            _ => Err(WindowConfigPackLoadDiagnostic::TypedState),
        }
    }

    fn into_field(value: BuiltValue) -> Result<semio_framework_dsl_record::FieldValue, WindowConfigPackLoadDiagnostic> {
        match value {
            BuiltValue::Field(value) => Ok(value),
            _ => Err(WindowConfigPackLoadDiagnostic::TypedState),
        }
    }

    fn emit(&mut self, mut value: BuiltValue) -> Result<(), WindowConfigPackLoadDiagnostic> {
        while let Some(&(wrapper, at)) = self.wrappers.last() {
            if at != self.stack.len() {
                break;
            }
            self.wrappers.pop();
            value = match wrapper {
                ValueWrapper::Block => BuiltValue::Field(semio_framework_dsl_record::FieldValue::Block(Box::new(Self::into_field(value)?))),
                ValueWrapper::Dynamic => BuiltValue::Field(semio_framework_dsl_record::FieldValue::Value(Self::into_dsl(value)?)),
            };
        }
        match self.stack.last_mut() {
            Some(ValueFrame::Record { fields, field, .. }) => {
                let id = field.take().ok_or(WindowConfigPackLoadDiagnostic::TypedState)?;
                if fields.insert(id, Self::into_field(value)?).is_some() {
                    return Err(WindowConfigPackLoadDiagnostic::TypedState);
                }
            }
            Some(ValueFrame::Sequence { values, .. }) => values.push(value),
            Some(ValueFrame::Map { values, key, .. }) => values.push((key.take().ok_or(WindowConfigPackLoadDiagnostic::TypedState)?, value)),
            Some(ValueFrame::Statements { values, keyword, .. }) => {
                let keyword = keyword.take().ok_or(WindowConfigPackLoadDiagnostic::TypedState)?;
                let record = match Self::into_field(value)? {
                    semio_framework_dsl_record::FieldValue::Record(record) => record,
                    _ => return Err(WindowConfigPackLoadDiagnostic::TypedState),
                };
                values.push((keyword, record));
            }
            Some(ValueFrame::Bytes { .. }) => return Err(WindowConfigPackLoadDiagnostic::TypedState),
            None => {
                let field = Self::into_field(value)?;
                let state = <O::State as semio_framework_dsl_record::DslField>::from_value(&field).map_err(|_| WindowConfigPackLoadDiagnostic::TypedState)?;
                if self.root.replace(state).is_some() {
                    return Err(WindowConfigPackLoadDiagnostic::TypedState);
                }
            }
        }
        Ok(())
    }

    fn begin_container(&mut self, kind: store::mounted_pack_rt::RetainedValueContainer, count: u64) -> Result<(), WindowConfigPackLoadDiagnostic> {
        let expected = self.expected()?;
        let count = usize::try_from(count).map_err(|_| WindowConfigPackLoadDiagnostic::Capacity)?;
        match kind {
            store::mounted_pack_rt::RetainedValueContainer::Record => {
                let root = self.stack.is_empty() && self.root.is_none();
                let spec = if root { None } else { Self::child_record_spec(&expected) };
                let mut entries = Vec::new();
                entries.try_reserve_exact(count.max((if root { Some(&self.spec) } else { spec.as_ref() }).map_or(0, |spec| spec.fields.len()))).map_err(|_| WindowConfigPackLoadDiagnostic::Capacity)?;
                let fields = semio_framework_dsl_record::RecordFields::from_empty_slots(entries);
                self.stack.push(ValueFrame::Record { kind, root, spec, fields, field: None });
            }
            store::mounted_pack_rt::RetainedValueContainer::Tuple | store::mounted_pack_rt::RetainedValueContainer::List | store::mounted_pack_rt::RetainedValueContainer::PackedF64 | store::mounted_pack_rt::RetainedValueContainer::PackedVarint => {
                let mut values = Vec::new();
                values.try_reserve_exact(count).map_err(|_| WindowConfigPackLoadDiagnostic::Capacity)?;
                self.stack.push(ValueFrame::Sequence { kind, element: Self::child_element(&expected), values });
            }
            store::mounted_pack_rt::RetainedValueContainer::Map => {
                let mut values = Vec::new();
                values.try_reserve_exact(count).map_err(|_| WindowConfigPackLoadDiagnostic::Capacity)?;
                self.stack.push(ValueFrame::Map { dsl: expected.dsl, element: Self::child_element(&expected), values, key: None });
            }
            store::mounted_pack_rt::RetainedValueContainer::Statements => {
                let variants = match expected.shape {
                    Some(semio_framework_dsl_record::BorrowedShape::Statements(variants)) => variants,
                    _ => &[],
                };
                let mut values = Vec::new();
                values.try_reserve_exact(count).map_err(|_| WindowConfigPackLoadDiagnostic::Capacity)?;
                self.stack.push(ValueFrame::Statements { variants, values, keyword: None });
            }
            store::mounted_pack_rt::RetainedValueContainer::ChunkedBytes | store::mounted_pack_rt::RetainedValueContainer::Table | store::mounted_pack_rt::RetainedValueContainer::Wire => return Err(WindowConfigPackLoadDiagnostic::TypedState),
        }
        Ok(())
    }

    fn end_container(&mut self, kind: store::mounted_pack_rt::RetainedValueContainer) -> Result<(), WindowConfigPackLoadDiagnostic> {
        if !self.pending_wrappers().is_empty() {
            return Err(WindowConfigPackLoadDiagnostic::TypedState);
        }
        let frame = self.stack.pop().ok_or(WindowConfigPackLoadDiagnostic::TypedState)?;
        let value = match frame {
            ValueFrame::Record { kind: expected, root, spec, mut fields, field: None } if expected == kind => {
                if let Some(spec) = if root { Some(&self.spec) } else { spec.as_ref() } {
                    for field in spec.fields {
                        if !fields.contains_key(&field.id) {
                            fields.insert(field.id, semio_framework_dsl_record::FieldValue::Absent);
                        }
                    }
                }
                BuiltValue::Field(semio_framework_dsl_record::FieldValue::Record(semio_framework_dsl_record::RecordValue { fields }))
            }
            ValueFrame::Sequence { kind: expected, values, .. } if expected == kind => {
                let values = values.into_iter().map(Self::into_field).collect::<Result<Vec<_>, _>>()?;
                let tuple = matches!(kind, store::mounted_pack_rt::RetainedValueContainer::Tuple) || Self::shape_is_tuple_valued(self.expected()?.shape.as_ref());
                BuiltValue::Field(if tuple { semio_framework_dsl_record::FieldValue::Tuple(values) } else { semio_framework_dsl_record::FieldValue::List(values) })
            }
            ValueFrame::Map { dsl: true, values, key: None, .. } if kind == store::mounted_pack_rt::RetainedValueContainer::Map => {
                BuiltValue::Dsl(semio_framework_value::DslValue::Object(values.into_iter().map(|(key, value)| Self::into_dsl(value).map(|value| (key, value))).collect::<Result<Vec<_>, _>>()?))
            }
            ValueFrame::Map { dsl: false, values, key: None, .. } if kind == store::mounted_pack_rt::RetainedValueContainer::Map => {
                BuiltValue::Field(semio_framework_dsl_record::FieldValue::Map(values.into_iter().map(|(key, value)| Self::into_field(value).map(|value| (key, value))).collect::<Result<Vec<_>, _>>()?))
            }
            ValueFrame::Statements { values, keyword: None, .. } if kind == store::mounted_pack_rt::RetainedValueContainer::Statements => BuiltValue::Field(semio_framework_dsl_record::FieldValue::Statements(values)),
            _ => return Err(WindowConfigPackLoadDiagnostic::TypedState),
        };
        self.emit(value)
    }

    fn scalar_signed(&self, value: i64) -> BuiltValue {
        let expected = self.expected().ok();
        if expected.as_ref().is_some_and(|expected| expected.dsl) {
            BuiltValue::Dsl(semio_framework_value::DslValue::int(value))
        } else if expected.as_ref().and_then(|expected| expected.shape.as_ref()).is_some_and(|shape| matches!(shape, semio_framework_dsl_record::BorrowedShape::UInt | semio_framework_dsl_record::BorrowedShape::Count)) && value >= 0 {
            BuiltValue::Field(semio_framework_dsl_record::FieldValue::UInt(value as u64))
        } else if let Some(semio_framework_dsl_record::BorrowedShape::Enum(_)) = expected.as_ref().and_then(|expected| expected.shape.as_ref()) {
            BuiltValue::Field(semio_framework_dsl_record::FieldValue::Enum(value as u32))
        } else {
            BuiltValue::Field(semio_framework_dsl_record::FieldValue::Int(value))
        }
    }

    fn accept(&mut self, token: Mounted, catalog: &store::mounted_pack_rt::RetainedPackCatalogCursor) -> Result<(), WindowConfigPackLoadDiagnostic> {
        use store::mounted_pack_rt::{RetainedValueRole as Role, RetainedValueToken as Token};
        match token {
            Token::Tag { value: tag @ (0x00..=0x02), .. } => {
                let expected = self.expected()?;
                let value = match (tag, expected.dsl) {
                    (0x00, false) => BuiltValue::Field(semio_framework_dsl_record::FieldValue::Absent),
                    (0x01, false) => BuiltValue::Field(semio_framework_dsl_record::FieldValue::Bool(false)),
                    (0x02, false) => BuiltValue::Field(semio_framework_dsl_record::FieldValue::Bool(true)),
                    (0x01, true) => BuiltValue::Dsl(semio_framework_value::DslValue::Bool(false)),
                    (0x02, true) => BuiltValue::Dsl(semio_framework_value::DslValue::Bool(true)),
                    _ => return Err(WindowConfigPackLoadDiagnostic::TypedState),
                };
                self.emit(value)?;
            }
            Token::Tag { value: 0x12, .. } if self.expected()?.dsl => self.emit(BuiltValue::Dsl(semio_framework_value::DslValue::Null))?,
            Token::Tag { value: 0x0e, .. } => self.wrappers.push((ValueWrapper::Block, self.stack.len())),
            Token::Tag { value: 0x11, .. } => self.wrappers.push((ValueWrapper::Dynamic, self.stack.len())),
            Token::Tag { value, .. } if matches!(value, 0x03..=0x0d | 0x0f..=0x10 | 0x15..=0x17) => self.tag = Some(value),
            Token::Begin { kind, count } => {
                self.tag.take();
                self.begin_container(kind, count)?;
            }
            Token::Unsigned { role: Role::FieldId, value } => match self.stack.last_mut() {
                Some(ValueFrame::Record { field, .. }) if field.is_none() && value <= u16::MAX as u64 => *field = Some(value as u16),
                _ => return Err(WindowConfigPackLoadDiagnostic::TypedState),
            },
            Token::Unsigned { role: Role::Symbol, value } => {
                self.tag.take();
                self.begin_symbol(value, catalog)?;
            }
            Token::Unsigned { role: Role::StringLength, value } => {
                self.tag.take();
                self.begin_string()?;
                let owner = self.string.as_mut().expect("retained inline string was just created");
                owner.value.try_reserve_exact(usize::try_from(value).map_err(|_| WindowConfigPackLoadDiagnostic::Capacity)?).map_err(|_| WindowConfigPackLoadDiagnostic::Capacity)?;
                owner.remaining = Some(value);
                if value == 0 {
                    self.finish_string()?;
                }
            }
            Token::Unsigned { role: Role::BytesLength, value } => {
                self.tag.take();
                let mut bytes = Vec::new();
                bytes.try_reserve_exact(usize::try_from(value).map_err(|_| WindowConfigPackLoadDiagnostic::Capacity)?).map_err(|_| WindowConfigPackLoadDiagnostic::Capacity)?;
                if value == 0 {
                    self.emit(BuiltValue::Field(semio_framework_dsl_record::FieldValue::Bytes64(bytes)))?;
                } else {
                    self.stack.push(ValueFrame::Bytes { values: bytes, remaining: usize::try_from(value).map_err(|_| WindowConfigPackLoadDiagnostic::Capacity)? });
                }
            }
            Token::StringChar(value) => {
                let owner = self.string.as_mut().ok_or(WindowConfigPackLoadDiagnostic::TypedState)?;
                owner.value.push(value);
                let remaining = owner.remaining.as_mut().ok_or(WindowConfigPackLoadDiagnostic::TypedState)?;
                *remaining = remaining.checked_sub(value.len_utf8() as u64).ok_or(WindowConfigPackLoadDiagnostic::TypedState)?;
                if *remaining == 0 {
                    self.finish_string()?;
                }
            }
            Token::Byte(value) => match self.stack.last_mut() {
                Some(ValueFrame::Bytes { values, remaining }) => {
                    values.push(value);
                    *remaining = remaining.checked_sub(1).ok_or(WindowConfigPackLoadDiagnostic::TypedState)?;
                    if *remaining == 0 {
                        let ValueFrame::Bytes { values, remaining: 0 } = self.stack.pop().expect("retained byte owner remains topmost") else { unreachable!() };
                        self.emit(BuiltValue::Field(semio_framework_dsl_record::FieldValue::Bytes64(values)))?;
                    }
                }
                _ => return Err(WindowConfigPackLoadDiagnostic::TypedState),
            },
            Token::Signed(value) => {
                self.tag.take();
                let scalar = self.scalar_signed(value);
                self.emit(scalar)?;
            }
            Token::Unsigned { role: Role::Unsigned | Role::Integer | Role::Enum, value } => {
                let expected = self.expected()?;
                let built = if expected.dsl {
                    BuiltValue::Dsl(semio_framework_value::DslValue::uint(value))
                } else if matches!(expected.shape, Some(semio_framework_dsl_record::BorrowedShape::Enum(_))) || self.tag == Some(0x0a) {
                    BuiltValue::Field(semio_framework_dsl_record::FieldValue::Enum(u32::try_from(value).map_err(|_| WindowConfigPackLoadDiagnostic::TypedState)?))
                } else {
                    BuiltValue::Field(semio_framework_dsl_record::FieldValue::UInt(value))
                };
                self.tag.take();
                self.emit(built)?;
            }
            Token::F64(bits) => {
                self.tag.take();
                let expected = self.expected()?;
                let built = if expected.dsl { BuiltValue::Dsl(semio_framework_value::DslValue::float(f64::from_bits(bits))) } else { BuiltValue::Field(semio_framework_dsl_record::FieldValue::Float(f64::from_bits(bits))) };
                self.emit(built)?;
            }
            Token::End(kind) => self.end_container(kind)?,
            Token::Complete { .. } => {
                if !self.stack.is_empty() || !self.wrappers.is_empty() || self.string.is_some() || self.root.is_none() {
                    return Err(WindowConfigPackLoadDiagnostic::TypedState);
                }
                self.complete = true;
            }
            Token::Unsigned { role: Role::TableRows | Role::TableField | Role::Chunk, .. }
            | Token::WirePresence(_)
            | Token::WireNodePresence(_)
            | Token::WireLabelPresence(_)
            | Token::TablePresence { .. }
            | Token::TableBitmap { .. }
            | Token::Tag { .. } => return Err(WindowConfigPackLoadDiagnostic::TypedState),
            Token::Unsigned { role: Role::Count, .. } => return Err(WindowConfigPackLoadDiagnostic::TypedState),
        }
        Ok(())
    }

    fn take(&mut self) -> Option<O::State> {
        if !self.complete || self.handed_back {
            return None;
        }
        self.handed_back = true;
        self.root.take()
    }

    fn retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        self.retirement.as_ref().map_or(Ok(RetirementDemand { copy_bytes: std::mem::size_of::<RetainedTypedOwners<O>>(), depth: 1, ..Default::default() }), |owner| Ok(RetirementDemand { copy_bytes: owner.next_copy_byte_demand()?, capacity_bytes: owner.next_capacity_byte_demand(body)?, release_bytes: owner.next_release_byte_demand()?, depth: owner.next_depth_demand()? }))
    }
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        let demand = self.retirement_demands(grant.maximum_copy_bytes)?;
        if !super::registry::fits(grant, demand) { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if let Some(owner) = self.retirement.as_mut() { return owner.step(grant); }
        let owners = self.retirement_original.take().unwrap_or_else(|| RetainedTypedOwners::<O> {
            spec: std::mem::replace(&mut self.spec, semio_framework_dsl_record::BorrowedRecordSpec { keyword: None, layout: semio_framework_dsl_record::RecordLayout::Inline, fields: &[] }),
            stack: std::mem::take(&mut self.stack), wrappers: std::mem::take(&mut self.wrappers), string: self.string.take(), root: self.root.take(),
        });
        match ControlledRetirement::new(owners) {
            Ok(owner) => { self.retirement = Some(owner); self.handed_back = true; Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() })) },
            Err((error, owners)) => { *self.retirement_original = Some(owners); Err(error) },
        }
    }

    fn terminal_is_empty(&self) -> bool {
        self.handed_back && self.root.is_none() && self.stack.capacity() == 0 && self.wrappers.capacity() == 0 && self.string.is_none() && self.retirement_original.is_none() && self.retirement.as_ref().is_some_and(ControlledRetirement::terminal_is_empty)
    }
}

impl<O:WindowConfigOwner> semio_framework_value::retirement::RetireOwned for RetainedWindowConfigTypedState<O>{
 fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{
  use semio_framework_value::retirement::{sequence,deferred};let original=ManuallyDrop::new(self);
  unsafe{sequence(vec![deferred(std::ptr::read(&original.spec)),deferred(std::ptr::read(&original.stack)),deferred(std::ptr::read(&original.wrappers)),deferred(std::ptr::read(&original.string)),deferred(original.tag),deferred(std::ptr::read(&original.root)),deferred(original.complete),deferred(original.handed_back)])}
 }
 fn retirement_birth_bytes(&self)->Option<usize>{use semio_framework_value::retirement::{sequence_birth_bytes,deferred_birth_bytes_for};sequence_birth_bytes(&[deferred_birth_bytes_for(&self.spec),deferred_birth_bytes_for(&self.stack),deferred_birth_bytes_for(&self.wrappers),deferred_birth_bytes_for(&self.string),deferred_birth_bytes_for(&self.tag),deferred_birth_bytes_for(&self.root),deferred_birth_bytes_for(&self.complete),deferred_birth_bytes_for(&self.handed_back)])}
 fn controlled_retirement_supported()->bool{true}
}
impl<O: WindowConfigOwner> Drop for RetainedWindowConfigTypedState<O> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "retained typed window config reached Drop before handoff or terminal-empty retirement");
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RetainedStatePhase {
    Envelope,
    Ingress,
    Replay,
    Ready,
    Closing,
    Closed,
}

struct RetainedWindowConfigStateDecode<O: WindowConfigOwner> {
    active:ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    phase: RetainedStatePhase,
    inner_start: usize,
    admitted: usize,
    document_byte: Option<(u64, u8)>,
    source: ManuallyDrop<Option<store::mounted_pack_rt::RetainedPackSourceCursor>>,
    anchor: ManuallyDrop<Option<store::mounted_pack_rt::RetainedPackAnchorCursor>>,
    segment: ManuallyDrop<Option<store::mounted_pack_rt::RetainedPackSegmentCursor>>,
    catalog: ManuallyDrop<Option<store::mounted_pack_rt::RetainedPackCatalogCursor>>,
    value: ManuallyDrop<Option<store::mounted_pack_rt::RetainedValueCursor>>,
    typed: ManuallyDrop<Option<RetainedWindowConfigTypedState<O>>>,
    catalog_value: ManuallyDrop<Option<store::mounted_pack_rt::RetainedPackCatalog>>,
    source_complete: bool,
    anchor_ready: bool,
    segment_complete: bool,
    catalog_complete: bool,
    value_sealed: bool,
    value_complete: bool,
    state: ManuallyDrop<Option<O::State>>,
    state_retirement: Option<ControlledRetirement<O::State>>,
    digest: Option<[u8; 32]>,
    hasher: semio_framework_hash::Hasher,
    native: Option<semio_framework_value::native_decoding::NativeDecodeContinuation>,
}

impl<O: WindowConfigOwner> RetainedWindowConfigStateDecode<O> {
    fn new() -> Self {
        Self {
            active:ManuallyDrop::new(None),
            phase: RetainedStatePhase::Envelope,
            inner_start: 0,
            admitted: 0,
            document_byte: None,
            source: ManuallyDrop::new(None),
            anchor: ManuallyDrop::new(None),
            segment: ManuallyDrop::new(None),
            catalog: ManuallyDrop::new(None),
            value: ManuallyDrop::new(None),
            typed: ManuallyDrop::new(None),
            catalog_value: ManuallyDrop::new(None),
            source_complete: false,
            anchor_ready: false,
            segment_complete: false,
            catalog_complete: false,
            value_sealed: false,
            value_complete: false,
            state: ManuallyDrop::new(None),
            state_retirement: None,
            digest: None,
            hasher: semio_framework_hash::Hasher::new(),
            native: None,
        }
    }

    fn validate_envelope(&mut self, pack: &[u8], grant: RetainedCloneGrant) -> Result<bool, WindowConfigPackLoadDiagnostic> {
        let envelope_id = <O::State as store::ArtifactDsl>::envelope_id();
        let expected_token_bytes = envelope_id.len().checked_add(".pack v1".len()).ok_or(WindowConfigPackLoadDiagnostic::EnvelopeIdentity)?;
        let header_bytes = 12usize.checked_add(expected_token_bytes).ok_or(WindowConfigPackLoadDiagnostic::EnvelopeIdentity)?;
        if grant.maximum_items == 0 || grant.maximum_depth == 0 || grant.maximum_copy_bytes < header_bytes.saturating_add(std::mem::size_of::<RetainedWindowConfigTypedState<O>>()) {
            return Ok(false);
        }
        if pack.len() <= header_bytes || pack.get(..8) != Some(store::semio_format::BINARY_MAGIC.as_slice()) || pack.get(8..12).and_then(|bytes| <[u8; 4]>::try_from(bytes).ok()).map(u32::from_le_bytes) != Some(expected_token_bytes as u32) {
            return Err(WindowConfigPackLoadDiagnostic::EnvelopeIdentity);
        }
        let token = pack.get(12..header_bytes).ok_or(WindowConfigPackLoadDiagnostic::EnvelopeIdentity)?;
        let id = envelope_id.as_bytes();
        if token.get(..id.len()) != Some(id) || token.get(id.len()..) != Some(b".pack v1") {
            return Err(WindowConfigPackLoadDiagnostic::EnvelopeIdentity);
        }
        let inner_len = pack.len() - header_bytes;
        if inner_len > store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES {
            return Err(WindowConfigPackLoadDiagnostic::Capacity);
        }
        let pages = inner_len.div_ceil(store::mounted_pack_rt::RETAINED_PACK_PAGE_BYTES);
        if pages == 0 || pages > store::mounted_pack_rt::RETAINED_PACK_MAXIMUM_PAGES {
            return Err(WindowConfigPackLoadDiagnostic::Capacity);
        }
        let maximum_items = store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES;
        let maximum_allocation = store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES;
        let limits =
            || store::mounted_pack_rt::PackLimits { max_file_len: inner_len as u64, max_segment_len: inner_len as u64, max_symbols: maximum_items as u32, max_depth: 64, max_items: maximum_items as u64, max_total_alloc: maximum_allocation as u64 };
        *self.source = Some(store::mounted_pack_rt::RetainedPackSourceCursor::try_new(pages, inner_len, maximum_allocation).map_err(|_| WindowConfigPackLoadDiagnostic::Pack)?);
        *self.anchor = Some(store::mounted_pack_rt::RetainedPackAnchorCursor::new());
        *self.segment = Some(store::mounted_pack_rt::RetainedPackSegmentCursor::try_new(limits(), maximum_allocation).map_err(|_| WindowConfigPackLoadDiagnostic::Pack)?);
        *self.catalog = Some(store::mounted_pack_rt::RetainedPackCatalogCursor::try_new(limits(), maximum_items, inner_len, inner_len, maximum_items, maximum_allocation).map_err(|_| WindowConfigPackLoadDiagnostic::Pack)?);
        *self.value = Some(store::mounted_pack_rt::RetainedValueCursor::try_new(limits(), maximum_allocation).map_err(|_| WindowConfigPackLoadDiagnostic::Pack)?);
        let mut accepted = |progress: semio_framework_value::native_decoding::NativeDecodeProgress| progress.completed <= grant.maximum_items;
        let mut native = match self.native.take() {
            Some(receipt) => semio_framework_value::NativeDecodeControl::resume(receipt, &mut accepted).map_err(|_| WindowConfigPackLoadDiagnostic::TypedState)?,
            None => semio_framework_value::NativeDecodeControl::new_retained(&mut accepted),
        };
        native.admit_turn_capacity(grant.maximum_capacity_bytes).map_err(|_| WindowConfigPackLoadDiagnostic::Capacity)?;
        let typed = native.scoped_depth(grant.maximum_depth, |native| RetainedWindowConfigTypedState::<O>::new(native).map_err(|_| ValueError::literal(ValueRefusalKind::InvalidValue, "Window config requires its actual authored metadata producer")));
        self.native = Some(native.pause().map_err(|_| WindowConfigPackLoadDiagnostic::TypedState)?);
        *self.typed = Some(typed.map_err(|_| WindowConfigPackLoadDiagnostic::TypedState)?);
        self.hasher.update(&pack[..header_bytes]);
        self.inner_start = header_bytes;
        self.phase = RetainedStatePhase::Ingress;
        Ok(true)
    }

    fn retained_allocated_bytes(&self) -> usize {
        self.source.as_ref().map_or(0, store::mounted_pack_rt::RetainedPackSourceCursor::allocated_bytes)
            + self.segment.as_ref().map_or(0, store::mounted_pack_rt::RetainedPackSegmentCursor::allocated_bytes)
            + self.catalog.as_ref().map_or(0, store::mounted_pack_rt::RetainedPackCatalogCursor::allocated_bytes)
            + self.value.as_ref().map_or(0, store::mounted_pack_rt::RetainedValueCursor::allocated_bytes)
    }

    fn reserve_next(&mut self, maximum_capacity_bytes: usize) -> Result<bool, WindowConfigPackLoadDiagnostic> {
        let allocated = self.retained_allocated_bytes();
        let remaining = store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES.saturating_sub(allocated);
        if self.phase == RetainedStatePhase::Ingress {
            let source = self.source.as_mut().ok_or(WindowConfigPackLoadDiagnostic::Pack)?;
            if source.has_reserved_page() {
                return Ok(false);
            }
            let requested = source.next_allocation_bytes().map_err(|_| WindowConfigPackLoadDiagnostic::Pack)?;
            if requested > maximum_capacity_bytes || requested > remaining {
                return Ok(false);
            }
            let step = source.reserve_page(maximum_capacity_bytes.min(remaining)).map_err(|_| WindowConfigPackLoadDiagnostic::Capacity)?;
            return Ok(step.progressed);
        }
        if self.phase != RetainedStatePhase::Replay {
            return Ok(false);
        }
        let segment = self.segment.as_mut().ok_or(WindowConfigPackLoadDiagnostic::Pack)?;
        if let Some(requested) = segment.next_allocation_bytes() {
            if requested > maximum_capacity_bytes || requested > remaining {
                return Ok(false);
            }
            return segment.reserve_allocation(maximum_capacity_bytes.min(remaining)).map(|step| step.progressed).map_err(|_| WindowConfigPackLoadDiagnostic::Capacity);
        }
        let value = self.value.as_mut().ok_or(WindowConfigPackLoadDiagnostic::Pack)?;
        if let Some(requested) = value.next_allocation_bytes().map_err(|_| WindowConfigPackLoadDiagnostic::Pack)? {
            if requested > maximum_capacity_bytes || requested > remaining {
                return Ok(false);
            }
            return value.reserve_allocation(maximum_capacity_bytes.min(remaining)).map(|step| step.progressed).map_err(|_| WindowConfigPackLoadDiagnostic::Capacity);
        }
        let catalog = self.catalog.as_mut().ok_or(WindowConfigPackLoadDiagnostic::Pack)?;
        if let Some(requested) = catalog.next_allocation_bytes().map_err(|_| WindowConfigPackLoadDiagnostic::Pack)? {
            if requested > maximum_capacity_bytes || requested > remaining {
                return Ok(false);
            }
            return catalog.reserve_allocation(maximum_capacity_bytes.min(remaining)).map(|step| step.progressed).map_err(|_| WindowConfigPackLoadDiagnostic::Capacity);
        }
        Ok(false)
    }

    fn ingress(&mut self, pack: &[u8], grant: RetainedCloneGrant) -> Result<bool, WindowConfigPackLoadDiagnostic> {
        let start = self.inner_start + self.admitted;
        let remaining = pack.len().saturating_sub(start);
        if remaining == 0 {
            let source = self.source.as_mut().ok_or(WindowConfigPackLoadDiagnostic::Pack)?;
            source.seal().map_err(|_| WindowConfigPackLoadDiagnostic::Pack)?;
            self.phase = RetainedStatePhase::Replay;
            return Ok(true);
        }
        if self.reserve_next(grant.maximum_capacity_bytes)? {
            return Ok(true);
        }
        let source = self.source.as_mut().ok_or(WindowConfigPackLoadDiagnostic::Pack)?;
        if !source.has_reserved_page() {
            return Ok(false);
        }
        let len = remaining.min(store::mounted_pack_rt::RETAINED_PACK_PAGE_BYTES).min(grant.maximum_copy_bytes);
        if len == 0 {
            return Ok(false);
        }
        source.preflight_page(len).map_err(|_| WindowConfigPackLoadDiagnostic::Pack)?;
        let mut bytes = [0; store::mounted_pack_rt::RETAINED_PACK_PAGE_BYTES];
        bytes[..len].copy_from_slice(&pack[start..start + len]);
        self.hasher.update(&pack[start..start + len]);
        let page = store::mounted_pack_rt::RetainedPackPage::try_from_array(bytes, len).map_err(|_| WindowConfigPackLoadDiagnostic::Pack)?;
        source.admit_page(page).map_err(|_| WindowConfigPackLoadDiagnostic::Pack)?;
        self.admitted += len;
        Ok(true)
    }

    fn replay(&mut self, grant: RetainedCloneGrant) -> Result<bool, WindowConfigPackLoadDiagnostic> {
        if self.reserve_next(grant.maximum_capacity_bytes)? {
            return Ok(true);
        }
        if self.typed.as_mut().ok_or(WindowConfigPackLoadDiagnostic::TypedState)?.grant_symbol(self.catalog.as_ref().ok_or(WindowConfigPackLoadDiagnostic::Pack)?)? {
            return Ok(true);
        }
        if let Some((index, byte)) = self.document_byte {
            if self.value.as_ref().ok_or(WindowConfigPackLoadDiagnostic::Pack)?.ingress_ready() {
                self.document_byte = None;
                self.value.as_mut().expect("retained config value owner remains").admit_byte(index, byte).map_err(|_| WindowConfigPackLoadDiagnostic::Pack)?;
                return Ok(true);
            }
        }
        if !self.value_complete {
            if let Some(token) = self.value.as_mut().ok_or(WindowConfigPackLoadDiagnostic::Pack)?.grant().map_err(|_| WindowConfigPackLoadDiagnostic::Pack)? {
                self.value_complete = matches!(token, store::mounted_pack_rt::RetainedValueToken::Complete { .. });
                self.typed.as_mut().ok_or(WindowConfigPackLoadDiagnostic::TypedState)?.accept(token, self.catalog.as_ref().ok_or(WindowConfigPackLoadDiagnostic::Pack)?)?;
                return Ok(true);
            }
        }
        if self.catalog_complete && !self.value_sealed {
            let bytes = self.catalog.as_ref().ok_or(WindowConfigPackLoadDiagnostic::Pack)?.document_bytes();
            self.value.as_mut().ok_or(WindowConfigPackLoadDiagnostic::Pack)?.seal(bytes).map_err(|_| WindowConfigPackLoadDiagnostic::Pack)?;
            self.value_sealed = true;
            return Ok(true);
        }
        if self.catalog.as_ref().is_some_and(store::mounted_pack_rt::RetainedPackCatalogCursor::has_pending_input) {
            if let Some(event) = self.catalog.as_mut().ok_or(WindowConfigPackLoadDiagnostic::Pack)?.grant().map_err(|_| WindowConfigPackLoadDiagnostic::Pack)? {
                match event {
                    store::mounted_pack_rt::RetainedPackCatalogEvent::DocumentByte { index, value, .. } => self.document_byte = Some((index, value)),
                    store::mounted_pack_rt::RetainedPackCatalogEvent::Complete => self.catalog_complete = true,
                    _ => {}
                }
            }
            return Ok(true);
        }
        let segment_can_admit = self.segment.as_ref().ok_or(WindowConfigPackLoadDiagnostic::Pack)?.preflight().is_ok();
        if self.document_byte.is_none() && !self.segment_complete && (!segment_can_admit || self.source_complete) {
            if let Some(event) = self.segment.as_mut().ok_or(WindowConfigPackLoadDiagnostic::Pack)?.grant().map_err(|_| WindowConfigPackLoadDiagnostic::Pack)? {
                self.segment_complete = matches!(event, store::mounted_pack_rt::RetainedPackSegmentEvent::PackComplete { .. });
                let catalog = self.catalog.as_mut().ok_or(WindowConfigPackLoadDiagnostic::Pack)?;
                catalog.admit(event).map_err(|_| WindowConfigPackLoadDiagnostic::Pack)?;
                if let Some(event) = catalog.grant().map_err(|_| WindowConfigPackLoadDiagnostic::Pack)? {
                    match event {
                        store::mounted_pack_rt::RetainedPackCatalogEvent::DocumentByte { index, value, .. } => self.document_byte = Some((index, value)),
                        store::mounted_pack_rt::RetainedPackCatalogEvent::Complete => self.catalog_complete = true,
                        _ => {}
                    }
                }
                return Ok(true);
            }
        }
        if !self.source_complete && segment_can_admit {
            if let Some(event) = self.source.as_mut().ok_or(WindowConfigPackLoadDiagnostic::Pack)?.grant().map_err(|_| WindowConfigPackLoadDiagnostic::Pack)? {
                self.source_complete = matches!(event, store::mounted_pack_rt::RetainedPackSourceEvent::Complete { .. });
                self.anchor.as_mut().ok_or(WindowConfigPackLoadDiagnostic::Pack)?.grant(Some(event)).map_err(|_| WindowConfigPackLoadDiagnostic::Pack)?;
                self.segment.as_mut().ok_or(WindowConfigPackLoadDiagnostic::Pack)?.admit(event).map_err(|_| WindowConfigPackLoadDiagnostic::Pack)?;
                return Ok(true);
            }
        }
        if self.source_complete && !self.anchor_ready {
            self.anchor_ready = self.anchor.as_mut().ok_or(WindowConfigPackLoadDiagnostic::Pack)?.grant(None).map_err(|_| WindowConfigPackLoadDiagnostic::Pack)?;
            return Ok(true);
        }
        if self.anchor_ready && self.catalog_complete && self.value_complete && self.catalog_value.is_none() {
            let superblock = self.anchor.as_mut().ok_or(WindowConfigPackLoadDiagnostic::Pack)?.take().ok_or(WindowConfigPackLoadDiagnostic::Pack)?;
            *self.catalog_value = self.catalog.as_mut().ok_or(WindowConfigPackLoadDiagnostic::Pack)?.take(superblock).map_err(|_| WindowConfigPackLoadDiagnostic::Pack)?;
            *self.state = self.typed.as_mut().ok_or(WindowConfigPackLoadDiagnostic::TypedState)?.take();
            if self.state.is_none() {
                return Err(WindowConfigPackLoadDiagnostic::TypedState);
            }
            self.digest = Some(*self.hasher.finalize().as_bytes());
            self.phase = RetainedStatePhase::Ready;
            return Ok(true);
        }
        Ok(false)
    }

    fn advance(&mut self, pack: &[u8], grant: RetainedCloneGrant) -> Result<bool, WindowConfigPackLoadDiagnostic> {
        if grant.maximum_items == 0 {
            return Ok(false);
        }
        match self.phase {
            RetainedStatePhase::Envelope => self.validate_envelope(pack, grant),
            RetainedStatePhase::Ingress => self.ingress(pack, grant),
            RetainedStatePhase::Replay => self.replay(grant),
            RetainedStatePhase::Ready => Ok(true),
            RetainedStatePhase::Closing | RetainedStatePhase::Closed => Ok(false),
        }
    }

    fn take_ready(&mut self) -> Option<(O::State, [u8; 32])> {
        if self.phase != RetainedStatePhase::Ready {
            return None;
        }
        Some((self.state.take()?, self.digest.take()?))
    }

    fn request_cancel(&mut self) {
        if let Some(source) = self.source.as_mut() {
            source.request_cancel();
        }
        self.phase = RetainedStatePhase::Closing;
    }

    fn retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        let frame = |copy_bytes| Ok(RetirementDemand { copy_bytes, depth: 1, ..Default::default() });
        if self.terminal_is_empty() { return Ok(Default::default()); }
        if self.document_byte.is_some() { return frame(std::mem::size_of::<(u64, u8)>()); }
        if self.catalog_value.is_some() { return frame(std::mem::size_of::<store::mounted_pack_rt::RetainedPackCatalog>()); }
        if self.state.is_some() { return frame(std::mem::size_of::<O::State>()); }
        if let Some(owner) = self.state_retirement.as_ref() { return controlled_demands(owner, body); }
        if let Some(typed) = self.typed.as_ref() { return if typed.terminal_is_empty() { frame(std::mem::size_of::<RetainedWindowConfigTypedState<O>>()) } else { typed.retirement_demands(body) }; }
        if let Some(owner) = self.value.as_ref() { return if owner.terminal_is_empty() { frame(std::mem::size_of::<store::mounted_pack_rt::RetainedValueCursor>()) } else { Ok(owner.retirement_demands()) }; }
        if let Some(owner) = self.catalog.as_ref() { return if owner.terminal_is_empty() { frame(std::mem::size_of::<store::mounted_pack_rt::RetainedPackCatalogCursor>()) } else { owner.retirement_demands() }; }
        if let Some(owner) = self.segment.as_ref() { return if owner.terminal_is_empty() { frame(std::mem::size_of::<store::mounted_pack_rt::RetainedPackSegmentCursor>()) } else { Ok(owner.retirement_demands()) }; }
        if self.anchor.is_some() { return frame(std::mem::size_of::<store::mounted_pack_rt::RetainedPackAnchorCursor>()); }
        if let Some(owner) = self.source.as_ref() { return if owner.terminal_is_empty() { frame(std::mem::size_of::<store::mounted_pack_rt::RetainedPackSourceCursor>()) } else { owner.retirement_demands() }; }
        frame(0)
    }

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, String> {
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        let demand = self.retirement_demands(grant.maximum_copy_bytes).map_err(ValueError::into_message)?;
        if !super::registry::fits(grant, demand) { return Ok(RetainedCloneStep::Progress(Default::default())); }
        self.request_cancel();
        let moved = || RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() });
        if self.document_byte.take().is_some() || self.catalog_value.take().is_some() { return Ok(moved()); }
        if let Some(state) = self.state.take() {
            return match ControlledRetirement::new(state) {
                Ok(owner) => { self.state_retirement = Some(owner); Ok(moved()) },
                Err((error, state)) => { *self.state = Some(state); Err(error.into_message()) },
            };
        }
        if let Some(owner) = self.state_retirement.as_mut() {
            let step = owner.step(grant).map_err(ValueError::into_message)?;
            if owner.terminal_is_empty() { self.state_retirement = None; }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(typed) = self.typed.as_mut() {
            if typed.terminal_is_empty() { drop(self.typed.take()); return Ok(moved()); }
            return typed.close_step(grant).map(|step| RetainedCloneStep::Progress(step.progress())).map_err(ValueError::into_message);
        }
        if let Some(value) = self.value.as_mut() {
            if value.terminal_is_empty() { drop(self.value.take()); return Ok(moved()); }
            let step = value.close_step(grant.maximum_items.min(1), grant.maximum_release_bytes).map_err(|error| error.to_string())?;
            return Ok(pack_close_receipt(step, demand));
        }
        if let Some(catalog) = self.catalog.as_mut() {
            if catalog.terminal_is_empty() { drop(self.catalog.take()); return Ok(moved()); }
            let step = catalog.close_step(grant.maximum_items.min(1), grant.maximum_release_bytes).map_err(|error| error.reason.to_string())?;
            return Ok(pack_close_receipt(step, demand));
        }
        if let Some(segment) = self.segment.as_mut() {
            if segment.terminal_is_empty() { drop(self.segment.take()); return Ok(moved()); }
            return Ok(pack_close_receipt(segment.close_step(grant.maximum_items.min(1), grant.maximum_release_bytes), demand));
        }
        if let Some(anchor) = self.anchor.as_mut() {
            anchor.close_step();
            drop(self.anchor.take());
            return Ok(moved());
        }
        if let Some(source) = self.source.as_mut() {
            if source.terminal_is_empty() { drop(self.source.take()); return Ok(moved()); }
            let step = source.close_step(grant.maximum_items.min(1), grant.maximum_release_bytes).map_err(|error| error.to_string())?;
            return Ok(pack_close_receipt(step, demand));
        }
        self.phase = RetainedStatePhase::Closed;
        Ok(RetainedCloneStep::Complete(Default::default()))
    }

    fn close_retained(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}
        let demand=self.retirement_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes{return Ok(RetainedCloneStep::Progress(Default::default()));}
        if grant.maximum_depth<demand.depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"window state decoder depth refused"));}
        self.request_cancel();
        if self.active.is_some(){return store::artifact_retirement_box_close_step(&mut self.active,grant).map(|step|RetainedCloneStep::Progress(step.progress()));}
        if self.state.is_some(){return store::artifact_retirement_admit_owned(&mut self.state,&mut self.active,grant);}
        if self.typed.is_some(){return store::artifact_retirement_admit_owned(&mut self.typed,&mut self.active,grant);}
        if self.document_byte.take().is_some()||self.catalog_value.take().is_some(){return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}));}
        let convert=|step|match step{store::mounted_pack_rt::RetainedPackCloseStep::Pending{released_items,released_bytes}=>RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:released_items,released_bytes,..Default::default()}),store::mounted_pack_rt::RetainedPackCloseStep::Complete=>RetainedCloneStep::Progress(Default::default())};
        if let Some(value)=self.value.as_mut(){let step=value.close_step(1,grant.maximum_release_bytes).map_err(|_|ValueError::literal(ValueRefusalKind::InvariantViolated,"window value retirement refused"))?;if matches!(step,store::mounted_pack_rt::RetainedPackCloseStep::Complete){if !value.terminal_is_empty(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"window value false terminal"));}self.value.take();}return Ok(convert(step));}
        if let Some(catalog)=self.catalog.as_mut(){let step=catalog.close_step(1,grant.maximum_release_bytes).map_err(|_|ValueError::literal(ValueRefusalKind::InvariantViolated,"window catalog retirement refused"))?;if matches!(step,store::mounted_pack_rt::RetainedPackCloseStep::Complete){if !catalog.terminal_is_empty(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"window catalog false terminal"));}self.catalog.take();}return Ok(convert(step));}
        if let Some(segment)=self.segment.as_mut(){let step=segment.close_step(1,grant.maximum_release_bytes);if matches!(step,store::mounted_pack_rt::RetainedPackCloseStep::Complete){if !segment.terminal_is_empty(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"window segment false terminal"));}self.segment.take();}return Ok(convert(step));}
        if let Some(anchor)=self.anchor.as_mut(){anchor.close_step();self.anchor.take();return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}));}
        if let Some(source)=self.source.as_mut(){let step=source.close_step(1,grant.maximum_release_bytes).map_err(|_|ValueError::literal(ValueRefusalKind::InvariantViolated,"window source retirement refused"))?;if matches!(step,store::mounted_pack_rt::RetainedPackCloseStep::Complete){if !source.terminal_is_empty(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"window source false terminal"));}self.source.take();}return Ok(convert(step));}
        self.phase=RetainedStatePhase::Closed;Ok(RetainedCloneStep::Complete(Default::default()))
    }
    fn terminal_is_empty(&self) -> bool {
        self.phase == RetainedStatePhase::Closed
            && self.active.is_none()
            && self.document_byte.is_none()
            && self.source.is_none()
            && self.anchor.is_none()
            && self.segment.is_none()
            && self.catalog.is_none()
            && self.value.is_none()
            && self.typed.is_none()
            && self.catalog_value.is_none()
            && self.state.is_none()
            && self.state_retirement.is_none()
    }
}

impl<O:WindowConfigOwner> store::ErasedSnapshotRetirement for RetainedWindowConfigStateDecode<O>{
 fn next_copy_byte_demand(&self)->Result<usize,ValueError>{self.retirement_demands(0).map(|demand|demand.copy_bytes)}
 fn next_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{self.retirement_demands(body).map(|demand|demand.capacity_bytes)}
 fn next_release_byte_demand(&self)->Result<usize,ValueError>{self.retirement_demands(0).map(|demand|demand.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{self.retirement_demands(0).map(|demand|demand.depth)}
 fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.close_retained(grant)}
 fn terminal_is_empty(&self)->bool{RetainedWindowConfigStateDecode::terminal_is_empty(self)}
}
impl<O: WindowConfigOwner> Drop for RetainedWindowConfigStateDecode<O> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "retained window config state decoder reached Drop before terminal-empty retirement");
    }
}

#[derive(semio_framework_value::RetireOwned)]
enum LoadOriginal<O: WindowConfigOwner> {
    State(O::State),
    History(store::HistoryLog),
    HistoryAndAuxiliary((Option<store::HistoryLog>, Vec<String>, Vec<String>)),
    Auxiliary((Vec<String>, Vec<String>)),
    Files((Vec<u8>, Vec<u8>, String)),
    Text(String),
    SharedText(semio_framework_value::SharedUtf8),
}

struct TypedWindowConfigPackLoad<O: WindowConfigOwner> {
    completed_states:ManuallyDrop<[Option<RetainedWindowConfigStateDecode<O>>;3]>,
    pending_files:ManuallyDrop<Option<store::ArtifactPackFiles>>,
    pending_address:ManuallyDrop<Option<String>>,
    opened_actor: ManuallyDrop<Option<protocol::ActorId>>,
    window_id: ManuallyDrop<Option<String>>,
    window_kind_id: ManuallyDrop<Option<String>>,
    expected_id: ManuallyDrop<Option<String>>,
    files: ManuallyDrop<Option<store::ArtifactPackFiles>>,
    state_decode: ManuallyDrop<Option<RetainedWindowConfigStateDecode<O>>>,
    initial: ManuallyDrop<Option<O::State>>,

    validation: ManuallyDrop<Option<O::State>>,
    current: ManuallyDrop<Option<O::State>>,
    initial_digest: Option<[u8; 32]>,
    history_decode: ManuallyDrop<Option<store::RetainedHistoryDecode>>,
    history: ManuallyDrop<Option<store::HistoryLog>>,
    hydration: ManuallyDrop<Option<store::RetainedConfigStoreHydration<O::State, O::Mutation>>>,
    owners: ManuallyDrop<Option<store::DocumentStoreOwners<O::State, O::Mutation>>>,
    candidate: ManuallyDrop<Option<WindowConfigPartition<O>>>,
    displaced: ManuallyDrop<Option<WindowConfigPartition<O>>>,
    active: Option<ControlledRetirement<LoadOriginal<O>>>,
    original: ManuallyDrop<Option<LoadOriginal<O>>>,
    registry_lifetime: u64,
    partition_generation: Option<u64>,
    phase: WindowConfigPackLoadPhase,
    diagnostic: Option<WindowConfigPackLoadDiagnostic>,
    completed_bytes: u64,
    total_bytes: u64,
    decoded_states: u8,
    committed: bool,
    terminal: bool,
}

impl<O: WindowConfigOwner> TypedWindowConfigPackLoad<O> {
    fn new(registry_lifetime: u64, partition_generation: Option<u64>, pack: WindowConfigPack, opened_actor: protocol::ActorId) -> Self {
        let WindowConfigPack { window_id, window_kind_id, files } = pack;
        let total_bytes = files.pack.len().saturating_mul(3).saturating_add(files.spr.len()).saturating_add(files.ops.len()) as u64;
        let expected_id = format!("window-config:{}:{window_id}", O::WINDOW_KIND_ID);
        let (owners, diagnostic) = match O::build_store_owners() {
            Ok(owners) => (Some(owners), None),
            Err(_) => (None, Some(WindowConfigPackLoadDiagnostic::Capacity)),
        };
        Self {
            completed_states:ManuallyDrop::new(std::array::from_fn(|_|None)),
            pending_files:ManuallyDrop::new(None),
            pending_address:ManuallyDrop::new(None),
            opened_actor: ManuallyDrop::new(Some(opened_actor)),
            window_id: ManuallyDrop::new(Some(window_id)),
            window_kind_id: ManuallyDrop::new(Some(window_kind_id)),
            expected_id: ManuallyDrop::new(Some(expected_id)),
            files: ManuallyDrop::new(Some(files)),
            state_decode: ManuallyDrop::new(Some(RetainedWindowConfigStateDecode::new())),
            initial: ManuallyDrop::new(None),

            validation: ManuallyDrop::new(None),
            current: ManuallyDrop::new(None),
            initial_digest: None,
            history_decode: ManuallyDrop::new(None),
            history: ManuallyDrop::new(None),
            hydration: ManuallyDrop::new(None),
            owners: ManuallyDrop::new(owners),
            candidate: ManuallyDrop::new(None),
            displaced: ManuallyDrop::new(None),
            active: None,
            original: ManuallyDrop::new(None),
            registry_lifetime,
            partition_generation,
            phase: if diagnostic.is_some() { WindowConfigPackLoadPhase::RetiringRejectedCandidate } else { WindowConfigPackLoadPhase::EnvelopeIdentity },
            diagnostic,
            completed_bytes: 0,
            total_bytes,
            decoded_states: 0,
            committed: false,
            terminal: false,
        }
    }

    fn progress_now(&self) -> WindowConfigPackLoadProgress {
        WindowConfigPackLoadProgress { phase: self.phase, completed_bytes: self.completed_bytes.min(self.total_bytes), total_bytes: self.total_bytes }
    }

    fn pending(&self) -> WindowConfigPackLoadStep {
        WindowConfigPackLoadStep::Pending(self.progress_now())
    }

    fn reject(&mut self, diagnostic: WindowConfigPackLoadDiagnostic) -> WindowConfigPackLoadStep {
        self.diagnostic.get_or_insert(diagnostic);
        self.phase = WindowConfigPackLoadPhase::RetiringRejectedCandidate;
        WindowConfigPackLoadStep::Rejected(self.diagnostic.unwrap())
    }

    fn map_hydration_diagnostic(diagnostic: store::ConfigStoreHydrationDiagnostic) -> WindowConfigPackLoadDiagnostic {
        match diagnostic {
            store::ConfigStoreHydrationDiagnostic::Identity => WindowConfigPackLoadDiagnostic::InnerIdentity,
            store::ConfigStoreHydrationDiagnostic::Malformed => WindowConfigPackLoadDiagnostic::History,
            store::ConfigStoreHydrationDiagnostic::Replay => WindowConfigPackLoadDiagnostic::Replay,
            store::ConfigStoreHydrationDiagnostic::Capacity => WindowConfigPackLoadDiagnostic::Capacity,
            store::ConfigStoreHydrationDiagnostic::Initialization => WindowConfigPackLoadDiagnostic::Replay,
            store::ConfigStoreHydrationDiagnostic::Cancelled => WindowConfigPackLoadDiagnostic::Cancelled,
        }
    }

    fn advance_decode(&mut self, grant: RetainedCloneGrant) -> WindowConfigPackLoadStep {
        let pack = &self.files.as_ref().expect("retained window config input remains").pack;
        let decoder = self.state_decode.as_mut().expect("retained window config state decoder remains");
        let before = decoder.admitted;
        match decoder.advance(pack, RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant}) {
            Ok(_) => {
                self.completed_bytes = self.completed_bytes.saturating_add(decoder.admitted.saturating_sub(before) as u64);
                self.phase = match decoder.phase {
                    RetainedStatePhase::Envelope => WindowConfigPackLoadPhase::EnvelopeIdentity,
                    RetainedStatePhase::Ingress => WindowConfigPackLoadPhase::PackIngress,
                    RetainedStatePhase::Replay => WindowConfigPackLoadPhase::PackReplay,
                    RetainedStatePhase::Ready => {
                        let (state, digest) = decoder.take_ready().expect("ready retained window config state remains");
                        if self.initial_digest.is_some_and(|expected| expected != digest) {
                            return self.reject(WindowConfigPackLoadDiagnostic::TypedState);
                        }
                        self.initial_digest.get_or_insert(digest);
                        match self.decoded_states {
                            0 => *self.initial = Some(state),
                            1 => *self.validation = Some(state),
                            2 => *self.current = Some(state),
                            _ => return self.reject(WindowConfigPackLoadDiagnostic::TypedState),
                        }
                        self.decoded_states += 1;
                        WindowConfigPackLoadPhase::PackRetirement
                    }
                    RetainedStatePhase::Closing | RetainedStatePhase::Closed => return self.reject(WindowConfigPackLoadDiagnostic::Pack),
                };
                self.pending()
            }
            Err(diagnostic) => self.reject(diagnostic),
        }
    }

    fn advance_pack_retirement(&mut self, grant: RetainedCloneGrant) -> WindowConfigPackLoadStep {
        let decoder = self.state_decode.as_mut().expect("retained window config state decoder remains during close");
        match decoder.close_step(grant) {
            Ok(RetainedCloneStep::Complete(_)) if decoder.terminal_is_empty() => {
                self.state_decode.take();
                if self.decoded_states < 3 {
                    *self.state_decode = Some(RetainedWindowConfigStateDecode::new());
                    self.phase = WindowConfigPackLoadPhase::EnvelopeIdentity;
                    return self.pending();
                }
                let spr_len = self.files.as_ref().expect("retained window config SPR input remains").spr.len();
                let limits = store::RetainedSprLimits { file_bytes: spr_len as u64, frame_body_bytes: spr_len as u64, records: store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES as u64 };
                match store::RetainedHistoryDecode::new_persisted_document(spr_len, limits) {
                    Ok(decoder) => {
                        *self.history_decode = Some(decoder);
                        self.phase = WindowConfigPackLoadPhase::HistoryReplay;
                        self.pending()
                    }
                    Err(_) => self.reject(WindowConfigPackLoadDiagnostic::History),
                }
            }
            Ok(RetainedCloneStep::Complete(_)) => self.reject(WindowConfigPackLoadDiagnostic::Retirement),
            Ok(_) => self.pending(),
            Err(_) => self.reject(WindowConfigPackLoadDiagnostic::Retirement),
        }
    }

    fn advance_history(&mut self, grant: RetainedCloneGrant) -> WindowConfigPackLoadStep {
        let bytes = &self.files.as_ref().expect("retained window config SPR input remains").spr;
        let decoder = self.history_decode.as_mut().expect("retained window config history decoder remains");
        match decoder.step(bytes, grant.maximum_copy_bytes, grant.maximum_items.min(1)) {
            Ok(store::RetainedHistoryDecodeStep::Pending { completed_bytes, .. }) => {
                self.completed_bytes = (self.files.as_ref().expect("retained window config input remains").pack.len() as u64).saturating_add(completed_bytes);
                self.pending()
            }
            Ok(store::RetainedHistoryDecodeStep::Ready) => {
                if !super::registry::fits(grant, Self::original_demand()) { return self.pending(); }
                let history = decoder.take_ready().expect("ready retained window config history remains");
                let auxiliary = decoder.take_auxiliary_owners();
                if !decoder.terminal_is_empty() {
                    return self.reject(WindowConfigPackLoadDiagnostic::Retirement);
                }

                *self.history = Some(history);
                if self.adopt_original(LoadOriginal::Auxiliary(auxiliary)).is_err() { return self.reject(WindowConfigPackLoadDiagnostic::Retirement); }
                self.phase = WindowConfigPackLoadPhase::InputRetirement;
                self.pending()
            }
            Err(_) => self.reject(WindowConfigPackLoadDiagnostic::History),
        }
    }

    fn adopt_original(&mut self, original: LoadOriginal<O>) -> Result<(), ValueError> {
        match ControlledRetirement::new(original) {
            Ok(owner) => { self.active = Some(owner); Ok(()) }
            Err((error, original)) => { *self.original = Some(original); Err(error) }
        }
    }

    fn drive_active(&mut self, grant: RetainedCloneGrant) -> Result<Option<RetainedCloneStep>, ValueError> {
        let Some(owner) = self.active.as_mut() else { return Ok(None) };
        let step = owner.step(grant)?;
        if owner.terminal_is_empty() { self.active = None; }
        Ok(Some(RetainedCloneStep::Progress(step.progress())))
    }

    fn original_demand() -> RetirementDemand { RetirementDemand { copy_bytes: std::mem::size_of::<LoadOriginal<O>>(), depth: 1, ..Default::default() } }

    fn retire_files(&mut self, grant: RetainedCloneGrant) -> Result<Option<RetainedCloneStep>, ValueError> {
        if self.files.is_none() { return Ok(None); }
        let demand = Self::original_demand();
        if !super::registry::fits(grant, demand) { return Ok(Some(RetainedCloneStep::Progress(Default::default()))); }
        let files = self.files.take().expect("granted original files remain");
        self.adopt_original(LoadOriginal::Files((files.pack, files.spr, files.ops)))?;
        Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() })))
    }

    fn advance_input_retirement(&mut self, grant: RetainedCloneGrant) -> WindowConfigPackLoadStep {
        match self.drive_active(grant) {
            Ok(Some(_)) => return self.pending(),
            Ok(None) => {}
            Err(_) => return self.reject(WindowConfigPackLoadDiagnostic::Retirement),
        }
        if self.hydration.is_none() {
            if grant.maximum_items == 0 || grant.maximum_depth == 0 {
                return self.pending();
            }
            let initial = self.initial.take().expect("decoded initial window config state remains");
            let pack = std::mem::take(&mut self.files.as_mut().expect("verified window config Pack remains").pack);
            let digest = self.initial_digest.take().expect("decoded window config digest remains");
            let genesis = semio_framework_os_kernel::os_vcs::io::binary::genesis::AdmittedArtifactGenesis::from_verified_pack(initial, pack, digest);
            let validation = self.validation.take().expect("decoded validation window config state remains");
            let current = self.current.take().expect("decoded current window config state remains");
            let history = self.history.take().expect("decoded window config history remains");
            let expected_id = self.expected_id.take().expect("exact window config partition id remains");
            let owners = self.owners.take().expect("window config store owners remain");
            *self.hydration = Some(store::RetainedConfigStoreHydration::from_snapshots(
                genesis,
                validation,
                current,
                history,
                expected_id,
                O::SCHEMA.to_string(),
                owners,
                self.partition_generation.map_or(0, |generation| generation.saturating_add(1)),
                O::MAXIMUM_PUBLICATION_BYTES,
                self.opened_actor.take().expect("retained window load retains its opened actor before hydration"),
            ));
            return self.pending();
        }
        match self.retire_files(grant) { Ok(Some(_)) => return self.pending(), Ok(None) => {}, Err(_) => return self.reject(WindowConfigPackLoadDiagnostic::Retirement) }
        self.phase = WindowConfigPackLoadPhase::StoreHydration;
        self.pending()
    }

    fn advance_hydration(&mut self, grant: RetainedCloneGrant) -> WindowConfigPackLoadStep {
        let hydration = self.hydration.as_mut().expect("retained window config hydration remains");
        match hydration.advance(grant) {
            store::ConfigStoreHydrationStep::Pending(_) => self.pending(),
            store::ConfigStoreHydrationStep::Rejected(diagnostic) => self.reject(Self::map_hydration_diagnostic(diagnostic)),
            store::ConfigStoreHydrationStep::Ready(store) => {

                *self.candidate = Some(WindowConfigPartition { store: *store, disposer: Some(O::build_store_disposer()), pending_preview:None,pending_preview_reads:None,pending_preview_projection:None,pending_preview_projection_read:None,preview_retirement:None,returned_read_retirement:None });
                self.phase = WindowConfigPackLoadPhase::Ready;
                WindowConfigPackLoadStep::Ready
            }
        }
    }

    fn advance_inner(&mut self, grant: RetainedCloneGrant) -> WindowConfigPackLoadStep {
        if let Some(diagnostic) = self.diagnostic {
            return WindowConfigPackLoadStep::Rejected(diagnostic);
        }
        if grant.maximum_items == 0 || grant.maximum_depth == 0 {
            return self.pending();
        }
        match self.phase {
            WindowConfigPackLoadPhase::EnvelopeIdentity | WindowConfigPackLoadPhase::PackIngress | WindowConfigPackLoadPhase::PackReplay => self.advance_decode(grant),
            WindowConfigPackLoadPhase::PackRetirement => self.advance_pack_retirement(grant),
            WindowConfigPackLoadPhase::HistoryReplay => self.advance_history(grant),
            WindowConfigPackLoadPhase::InputRetirement => self.advance_input_retirement(grant),
            WindowConfigPackLoadPhase::StoreHydration => self.advance_hydration(grant),
            WindowConfigPackLoadPhase::Ready => WindowConfigPackLoadStep::Ready,
            WindowConfigPackLoadPhase::RetiringRejectedCandidate => WindowConfigPackLoadStep::Rejected(self.diagnostic.unwrap_or(WindowConfigPackLoadDiagnostic::Cancelled)),
            WindowConfigPackLoadPhase::RetiringDisplacedStore => self.pending(),
            WindowConfigPackLoadPhase::Complete => WindowConfigPackLoadStep::Complete,
        }
    }

    fn reject_stale(&mut self) -> WindowConfigPackLoadStep {
        self.reject(WindowConfigPackLoadDiagnostic::Stale)
    }

    fn install(&mut self, partitions: &mut WindowRegistry<String, WindowConfigPartition<O>>, registry_lifetime: u64) -> WindowConfigPackLoadStep {
        if self.phase != WindowConfigPackLoadPhase::Ready || self.registry_lifetime != registry_lifetime {
            return self.reject_stale();
        }
        let window_id = self.window_id.as_ref().expect("ready window config load retains its address");
        let live_generation = partitions.get(window_id).map(|partition| partition.store.generation());
        if live_generation != self.partition_generation {
            return self.reject_stale();
        }
        let candidate = self.candidate.take().expect("ready window config candidate remains");
        *self.displaced = partitions.insert(window_id.clone(), candidate);
        self.committed = true;
        self.phase = WindowConfigPackLoadPhase::RetiringDisplacedStore;
        self.pending()
    }

    fn partition_demands(partition: &WindowConfigPartition<O>, body: usize) -> Result<RetirementDemand, ValueError> {
        if partition.preview_retirement_pending() { return partition.preview_retirement_demand(); }
        partition.disposer.as_ref().map_or(Ok(RetirementDemand { copy_bytes: std::mem::size_of::<WindowConfigPartition<O>>(), depth: 1, ..Default::default() }), |owner| {
            if owner.terminal_is_empty(&partition.store) { Ok(RetirementDemand { release_bytes: std::mem::size_of_val(owner.as_ref()), depth: 1, ..Default::default() }) }
            else { owner.retirement_demands(&partition.store, body) }
        })
    }

    fn close_partition(slot: &mut Option<WindowConfigPartition<O>>, grant: RetainedCloneGrant, demand: RetirementDemand) -> Result<PluginLifecycleStep, String> {
        let partition = slot.as_mut().expect("granted original partition remains");
        if partition.preview_retirement_pending() { return partition.preview_retirement_step(grant).map(|step| PluginLifecycleStep::retained(step, false)).map_err(ValueError::into_message); }
        if let Some(owner) = partition.disposer.as_mut() {
            if !owner.terminal_is_empty(&partition.store) { return owner.close_step(&mut partition.store, grant).map(|step| match step { PluginLifecycleStep::Complete(progress) => PluginLifecycleStep::Progress(progress), other => other }).map_err(|fault| fault.message); }
            drop(partition.disposer.take());
            return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: demand.release_bytes, ..Default::default() }));
        }
        drop(slot.take());
        Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }))
    }

    fn retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        let frame = |copy_bytes| Ok(RetirementDemand { copy_bytes, depth: 1, ..Default::default() });
        if self.terminal { return Ok(Default::default()); }
        if self.original.is_some() { return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "retained original window input has no controlled authority")); }
        if let Some(owner) = self.active.as_ref() { return controlled_demands(owner, body); }
        if let Some(owner) = self.state_decode.as_ref() { return if owner.terminal_is_empty() { frame(std::mem::size_of::<RetainedWindowConfigStateDecode<O>>()) } else { owner.retirement_demands(body) }; }
        if self.history_decode.is_some() { return frame(std::mem::size_of::<store::RetainedHistoryDecode>().checked_add(Self::original_demand().copy_bytes).ok_or_else(|| ValueError::literal(ValueRefusalKind::WorkLimit, "window history handoff size overflow"))?); }
        if let Some(owner) = self.hydration.as_ref() {
            return if store::ErasedSnapshotRetirement::terminal_is_empty(owner) { frame(std::mem::size_of::<store::RetainedConfigStoreHydration<O::State, O::Mutation>>()) }
            else { Ok(RetirementDemand { copy_bytes: store::ErasedSnapshotRetirement::next_copy_byte_demand(owner)?, capacity_bytes: store::ErasedSnapshotRetirement::next_capacity_byte_demand(owner, body)?, release_bytes: store::ErasedSnapshotRetirement::next_release_byte_demand(owner)?, depth: store::ErasedSnapshotRetirement::next_depth_demand(owner)? }) };
        }
        if let Some(partition) = self.displaced.as_ref().or(self.candidate.as_ref()) { return Self::partition_demands(partition, body); }
        if self.current.is_some() || self.validation.is_some() || self.initial.is_some() || self.history.is_some() || self.files.is_some() { return Ok(Self::original_demand()); }
        if let Some(owner) = self.owners.as_ref() { return if owner.uninstalled_owners_terminal_is_empty() { frame(std::mem::size_of::<store::DocumentStoreOwners<O::State, O::Mutation>>()) } else { owner.uninstalled_owners_demands(body) }; }
        if self.opened_actor.is_some() || self.expected_id.is_some() || self.window_kind_id.is_some() || self.window_id.is_some() { return Ok(Self::original_demand()); }
        Ok(RetirementDemand { depth: 1, ..Default::default() })
    }

    fn close_inner(&mut self, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, String> {
        if self.terminal { return Ok(PluginLifecycleStep::Complete(Default::default())); }
        let demand = self.retirement_demands(grant.maximum_copy_bytes).map_err(ValueError::into_message)?;
        if !super::registry::fits(grant, demand) { return Ok(PluginLifecycleStep::Progress(Default::default())); }
        self.diagnostic.get_or_insert(WindowConfigPackLoadDiagnostic::Cancelled);
        if let Some(step) = self.drive_active(grant).map_err(ValueError::into_message)? { return Ok(PluginLifecycleStep::retained(step, false)); }
        if let Some(owner) = self.state_decode.as_mut() {
            if !owner.terminal_is_empty() { return owner.close_step(grant).map(|step| PluginLifecycleStep::retained(step, false)); }
            drop(self.state_decode.take());
            return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }));
        }
        if let Some(owner) = self.history_decode.as_mut() {
            let history = owner.take_partial();
            let (dictionary, edits) = owner.take_auxiliary_owners();
            assert!(owner.terminal_is_empty());
            drop(self.history_decode.take());
            self.adopt_original(LoadOriginal::HistoryAndAuxiliary((history, dictionary, edits))).map_err(ValueError::into_message)?;
            return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }));
        }
        if let Some(owner) = self.hydration.as_mut() {
            if !store::ErasedSnapshotRetirement::terminal_is_empty(owner) { return store::ErasedSnapshotRetirement::close_step(owner, grant).map(|step| PluginLifecycleStep::retained(step, false)).map_err(ValueError::into_message); }
            drop(self.hydration.take());
            return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }));
        }
        if self.displaced.is_some() { return Self::close_partition(&mut self.displaced, grant, demand); }
        if self.candidate.is_some() { return Self::close_partition(&mut self.candidate, grant, demand); }
        if let Some(state) = self.current.take().or_else(|| self.validation.take()).or_else(|| self.initial.take()) {
            self.adopt_original(LoadOriginal::State(state)).map_err(ValueError::into_message)?;
        } else if let Some(history) = self.history.take() {
            self.adopt_original(LoadOriginal::History(history)).map_err(ValueError::into_message)?;
        } else if let Some(step) = self.retire_files(grant).map_err(ValueError::into_message)? {
            return Ok(PluginLifecycleStep::retained(step, false));
        } else if let Some(owner) = self.owners.as_mut() {
            if !owner.uninstalled_owners_terminal_is_empty() { return owner.close_uninstalled_owners_step(grant).map(|step| PluginLifecycleStep::retained(step, false)).map_err(ValueError::into_message); }
            drop(self.owners.take());
        } else if let Some(actor) = self.opened_actor.take() {
            self.adopt_original(LoadOriginal::SharedText(actor.0)).map_err(ValueError::into_message)?;
        } else if let Some(text) = self.expected_id.take().or_else(|| self.window_kind_id.take()).or_else(|| self.window_id.take()) {
            self.adopt_original(LoadOriginal::Text(text)).map_err(ValueError::into_message)?;
        } else {
            self.initial_digest = None;
            self.terminal = true;
            self.phase = WindowConfigPackLoadPhase::Complete;
            return Ok(PluginLifecycleStep::Complete(Default::default()));
        }
        Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }))
    }

    fn ownership_is_empty(&self) -> bool {
        self.terminal
            && self.completed_states.iter().all(Option::is_none)
            && self.pending_files.is_none()
            && self.pending_address.is_none()
            && self.opened_actor.is_none()
            && self.window_id.is_none()
            && self.window_kind_id.is_none()
            && self.expected_id.is_none()
            && self.files.is_none()
            && self.state_decode.is_none()
            && self.initial.is_none()

            && self.validation.is_none()
            && self.current.is_none()
            && self.initial_digest.is_none()
            && self.history_decode.is_none()
            && self.history.is_none()
            && self.hydration.is_none()
            && self.owners.is_none()
            && self.candidate.is_none()
            && self.displaced.is_none()
            && self.active.is_none()
            && self.original.is_none()
    }
}

impl<O: WindowConfigOwner> ErasedWindowConfigPackLoad for TypedWindowConfigPackLoad<O> {
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn window_kind_id(&self) -> &str {
        self.window_kind_id.as_deref().unwrap_or("")
    }

    fn registry_lifetime(&self) -> u64 {
        self.registry_lifetime
    }

    fn phase(&self) -> WindowConfigPackLoadPhase {
        self.phase
    }

    fn progress(&self) -> WindowConfigPackLoadProgress {
        self.progress_now()
    }

    fn diagnostic(&self) -> Option<WindowConfigPackLoadDiagnostic> {
        self.diagnostic
    }

    fn advance(&mut self, grant: RetainedCloneGrant) -> WindowConfigPackLoadStep {
        self.advance_inner(grant)
    }

    fn request_cancel(&mut self) {
        self.reject(WindowConfigPackLoadDiagnostic::Cancelled);
    }

    fn reject_stale(&mut self) -> WindowConfigPackLoadStep {
        TypedWindowConfigPackLoad::reject_stale(self)
    }

    fn retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> { TypedWindowConfigPackLoad::retirement_demands(self, body) }

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, String> { self.close_inner(grant) }

    fn terminal_is_empty(&self) -> bool {
        self.ownership_is_empty()
    }
}

impl<O: WindowConfigOwner> Drop for TypedWindowConfigPackLoad<O> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.ownership_is_empty(), "typed window config Pack load reached Drop before terminal-empty handoff or retirement");
    }
}

pub(super) fn begin_typed_window_config_pack_load<O: WindowConfigOwner>(registry_lifetime: u64, partition_generation: Option<u64>, pack: WindowConfigPack, opened_actor: protocol::ActorId) -> WindowConfigPackLoad {
    let over_bound = pack.files.pack.len() > O::MAXIMUM_PUBLICATION_BYTES;
    let mut load = TypedWindowConfigPackLoad::<O>::new(registry_lifetime, partition_generation, pack, opened_actor);
    if over_bound {
        load.reject(WindowConfigPackLoadDiagnostic::Capacity);
    }
    WindowConfigPackLoad { inner: ManuallyDrop::new(Some(Box::new(load))),closed_progress:WindowConfigPackLoadProgress{phase:WindowConfigPackLoadPhase::Complete,completed_bytes:0,total_bytes:0},closed_diagnostic:None }
}

pub(super) fn commit_typed_window_config_pack_load<O: WindowConfigOwner>(
    registry_lifetime: u64,
    partitions: &mut WindowRegistry<String, WindowConfigPartition<O>>,
    load: &mut dyn ErasedWindowConfigPackLoad,
) -> Result<WindowConfigPackLoadStep, WindowConfigPackLoadDiagnostic> {
    let typed = load.as_any_mut().downcast_mut::<TypedWindowConfigPackLoad<O>>().ok_or(WindowConfigPackLoadDiagnostic::Stale)?;
    Ok(typed.install(partitions, registry_lifetime))
}
