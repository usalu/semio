use semio_framework_ui_contract::*;
use semio_framework::*;
use semio_framework::kernel::*;
use semio_framework_plugin_abi::{ArtifactAnalysis, ArtifactAnalyzer, ArtifactBuilder, ArtifactChildren, ArtifactComposer, ArtifactComposition, ArtifactDecomposer, ArtifactDeserializer, ArtifactSerializer, DerivedArtifactSpec};
use semio_framework_plugin_decl_app::{ArtifactApp, NaturalFileDecodeCursor};
use semio_framework_artifact_reference::{ArtifactDialect, ArtifactRef};
use semio_framework_plugin_decl_surface::{ArtifactEditor, ArtifactViewer};
use semio_framework_plugin_decl_views::tool_contract::ArtifactInstanceOperationOwner;
use semio_framework_plugin_decl_app::tool_contract::{ArtifactOwnedToolJobFactory, ArtifactReservedJob};
use pack::snapshot::ArtifactPack;
use store::ArtifactStoreInitializationAuthority;
use semio_framework_plugin_decl_emission::{AsyncTaskRun, AsyncTaskSource, ChildEmit, Emit, ToolCommandCatalog};
use semio_framework_os_kernel::os_store::backbone::Backbone;
use semio_framework_plugin_decl_views::child_view::ChildContentRetainers;
use semio_framework_plugin_decl_emission::child_emit_preparation::{ChildEmitPreparation, ChildEmitPreparationStep};
use semio_framework_value::{DslValue, ValueError};
use semio_framework_diagnostic::FaultFrom;
use semio_framework_value_derive::{FromValue, ToValue};
use semio_framework_plugin_window_kits::{LabelAxes, WindowKit};
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_os_kernel::{MemberFactory, Mutation, SpaceMember};
use semio_framework_plugin_decl_views::{NoConfigMutation, NoDraftMutation};
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_plugin_builder::Ready;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::mutations::set_count::SetCount;
use semio_framework_os_kernel::io::SubsetValidator;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::mutations::TestMutation;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::TestSnapshot;
use semio_framework_plugin_decl_views::tool_run_contract::ToolRunRetargetableJob;
use serde_json::Value;
use semio_framework_plugin_window_config::{WindowConfigOwner, WindowConfigReplaceMutation};
use semio_framework_plugin_window_transient::window_transient::WindowTransientOwner;
use semio_framework_job::InteractiveJob as _;
use semio_framework_os_kernel as protocol;
use semio_framework_os_kernel as store;

/// 🎟️ One ordinary 4096-byte page turn on every independent axis.
fn child_page_grant() -> store::RetainedCloneGrant {
    store::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:4096,maximum_release_bytes:4096,maximum_depth:656}
}

/// 🎟️ One item that funds only the release axis, as the physical backing of one original allocation.
fn child_release_grant(bytes: usize) -> store::RetainedCloneGrant {
    store::RetainedCloneGrant { maximum_items: 1, maximum_release_bytes: bytes, maximum_depth: 64, ..Default::default() }
}

fn preparation_grant(items: usize, bytes: usize) -> store::RetainedCloneGrant {
    store::RetainedCloneGrant { maximum_items: items, maximum_copy_bytes: bytes, maximum_capacity_bytes: bytes, maximum_release_bytes: bytes, maximum_depth: 64 }
}

/// 🎟️ One item with no byte axis funded: every positive demand must yield.
fn child_zero_grant() -> store::RetainedCloneGrant {
    store::RetainedCloneGrant { maximum_items: 1, maximum_depth: 64, ..Default::default() }
}

#[test]
fn child_emission_private_input_request_retains_whole_page_backing_undergrant() {
    let fixture:Value=serde_json::from_str(include_str!("../../../🧩️composition/📨️emission/🌱️genesis/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let reference=&fixture["source"]["reference"];
    let expected=semio_framework_artifact_reference::ArtifactRef{
        artifact_id:reference["artifactId"].as_str().unwrap().into(),
        dialect:semio_framework_artifact_reference::ArtifactDialect{artifact_kind:reference["dialect"]["artifactKind"].as_str().unwrap().into(),standard:reference["dialect"]["standard"].as_str().unwrap().into(),subset:reference["dialect"]["subset"].as_str().unwrap().into()},
    };
    let source=serde_json::to_vec(&fixture["source"]).unwrap();
    let(mut pages,birth)=semio_framework_trace::observe_heap_allocations_on_this_thread(||store::os_store::owned_schema_decode::OwnedSchemaDecodePages::try_with_credits(store::os_store::owned_schema_decode::OwnedSchemaDecodeCredits{maximum_pages:1,maximum_bytes:store::os_store::owned_schema_decode::OWNED_SCHEMA_DECODE_PAGE_BYTES}).unwrap());
    let backing=pages.allocation_byte_demand();assert_eq!(birth.requested_bytes,backing);
    pages.admit_page(store::os_store::owned_schema_decode::OwnedSchemaDecodePage::try_from_slice(&source).unwrap()).unwrap();pages.seal().unwrap();
    let mut request=store::MemberOpenRequest::new(semio_framework_job::OperationId(1),semio_framework_job::Generation(1),1000,expected,None,pages,store::os_spr::ActorId(fixture["source"]["actor"].as_str().unwrap().into()));
    let release_grant=|bytes:usize|store::RetainedCloneGrant{maximum_items:1,maximum_release_bytes:bytes,maximum_depth:1,..Default::default()};
    request.close_step(release_grant(1)).unwrap();request.close_step(release_grant(source.len())).unwrap();request.close_step(release_grant(1)).unwrap();
    let(step,denied)=semio_framework_trace::observe_heap_allocations_on_this_thread(||request.close_step(release_grant(backing-1)).unwrap());
    assert_eq!(denied.released_bytes,0,"whole original page allocation must remain retained under one-below grant");
    assert_eq!(step,store::RetainedCloneStep::Progress(Default::default()));
    assert_eq!(request.next_release_byte_demand().unwrap(),backing);
    let(step,accepted)=semio_framework_trace::observe_heap_allocations_on_this_thread(||request.close_step(release_grant(backing)).unwrap());
    assert_eq!(accepted.released_bytes,backing);
    assert_eq!(step,store::RetainedCloneStep::Progress(store::RetainedCloneProgress{copied_items:1,released_bytes:backing,..Default::default()}));
    for _ in 0..1024{if request.terminal_is_empty(){break;}let bytes=request.next_release_byte_demand().unwrap();request.close_step(release_grant(bytes)).unwrap();}
    assert!(request.terminal_is_empty());
}

#[test]
fn child_emission_owned_preview_preserves_exact_wire_prefix() {
    use semio_framework_plugin_decl_emission::child_emit_preparation::{ChildEmitPreparation, ChildEmitPreparationStep};
    use semio_framework_plugin_decl_emission::Emit;
    let fixture:Value=serde_json::from_str(include_str!("../../../../🏪️store/🧩️composition/📨️emission/📦️owned/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let operations=fixture["values"].as_array().unwrap().iter().map(|value|TestMutation::SetCount(SetCount{value:value.as_i64().unwrap()as i32})).collect::<Vec<_>>();
    let expected=operations.iter().map(::protocol::OpBinary::encode_op).collect::<Result<Vec<_>,_>>().unwrap();
    let labels=operations.iter().map(protocol::SemanticMutation::<TestSnapshot>::label).collect::<Vec<_>>();
    let mut emit=Emit::<TestMutation>::default();
    emit.child_preparations.push_back(ChildEmitPreparation::of_owned::<TestSnapshot,_>("fixture","child",operations));
    let mut ready=false;
    for _ in 0..1000{if matches!(emit.prepare_child_preview_one(preparation_grant(1, 4096)).unwrap(),ChildEmitPreparationStep::Ready(_)){ready=true;break;}}
    assert!(ready&&emit.owned_child_emits.is_empty());
    assert_eq!(emit.child_emits.len(),1);
    assert_eq!(emit.child_emits[0].ops,expected);
    assert_eq!(emit.child_emits[0].labels,labels);
    assert_eq!(Value::from(semio_framework_value::ToValue::to_value(&emit.child_emits[0])),serde_json::to_value(&emit.child_emits[0]).unwrap());
    for _ in 0..4096{if emit.close_child_one(child_page_grant()).unwrap().is_none(){break;}}
    assert!(emit.close_child_one(child_page_grant()).unwrap().is_none());
}

#[test]
fn child_emission_emit_retains_applying_source_without_wire_handoff() {
    use semio_framework_plugin_decl_emission::child_emit_preparation::{ChildEmitPreparation, ChildEmitPreparationStep};
    use semio_framework_plugin_decl_emission::Emit;
    let fixture: Value = serde_json::from_str(include_str!("../../../../🏪️store/🧩️composition/📨️emission/📦️owned/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let operations = fixture["values"].as_array().unwrap().iter().map(|value| RefusingChildOperation { inner: TestMutation::SetCount(SetCount { value: value.as_i64().unwrap() as i32 }), refuse: true }).collect::<Vec<_>>();
    let pointer=operations.as_ptr();
    let mut emit = Emit::<TestMutation>::default();
    emit.child_preparations.push_back(ChildEmitPreparation::of_owned::<TestSnapshot, _>("fixture", "child", operations));
    let mut ready = false;
    for _ in 0..1000 {
        let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||emit.prepare_child_one(child_page_grant()).unwrap());
        assert!(!heap.overflowed);assert!(heap.requested_bytes+heap.released_bytes<=4096);
        if matches!(step, ChildEmitPreparationStep::Ready(_)) { ready = true; break; }
    }
    assert!(ready);
    assert!(emit.child_emits.is_empty(), "applying owned source cannot become an unused wire group");
    assert_eq!(emit.owned_child_emits.len(),1);
    let retained=&emit.owned_child_emits[0];
    let mutations=retained.mutations::<RefusingChildOperation>().unwrap();
    assert_eq!(mutations.as_ptr(),pointer);
    for(index,mutation)in mutations.iter().enumerate(){match &mutation.inner{TestMutation::SetCount(value)=>assert_eq!(value.value as i64,fixture["values"][index].as_i64().unwrap()),_=>panic!("applying source variant changed")};assert!(mutation.refuse);}
    let metadata=retained.metadata().unwrap();
    assert_eq!(Value::from(semio_framework_value::ToValue::to_value(metadata)),serde_json::to_value(metadata).unwrap());
    assert!(metadata.ops.is_empty()&&metadata.labels.is_empty());
    assert_eq!(emit.close_child_one(store::RetainedCloneGrant{maximum_items:0,..child_page_grant()}).unwrap(),Some(semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Progress(Default::default())));
    assert_eq!(emit.owned_child_emits[0].mutations::<RefusingChildOperation>().unwrap().as_ptr(),pointer);
    let mut last=None;
    for _ in 0..4096 {
        let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||emit.close_child_one(child_page_grant()).unwrap());
        assert!(!heap.overflowed);assert!(heap.requested_bytes+heap.released_bytes<=4096);
        if let Some(semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Progress(progress))=step{assert_eq!(heap.released_bytes,progress.released_bytes);}
        last=step;
        if step.is_none(){break;}
    }
    if last.is_some(){}
    assert!(emit.close_child_one(child_page_grant()).unwrap().is_none());
    assert!(emit.owned_child_emits.is_empty());
    assert_eq!(emit.owned_child_emits.capacity(),0);
}

#[test]
fn child_emission_owned_ready_transfers_original_typed_vector_without_wire_decoding() {
    use semio_framework_plugin_decl_emission::child_emit_preparation::{ChildEmitPreparation, ChildEmitPreparationStep};
    use semio_framework_value::retained_clone::RetainedCloneGrant;
    let fixture: Value = serde_json::from_str(include_str!("../../../../🏪️store/🧩️composition/📨️emission/📦️owned/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let operations = fixture["values"].as_array().unwrap().iter().map(|value| TestMutation::SetCount(SetCount { value: value.as_i64().unwrap() as i32 })).collect::<Vec<_>>();
    let pointer = operations.as_ptr();
    let mut preparation = ChildEmitPreparation::of_owned::<TestSnapshot, _>("fixture", "child", operations);
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 64, maximum_capacity_bytes: 4096, maximum_release_bytes: 4096, maximum_depth: 64 };
    assert!(matches!(preparation.step(preparation_grant(0, 4096)).unwrap(), ChildEmitPreparationStep::Pending(_)));
    let mut ready = false;
    for _ in 0..100 {
        if matches!(preparation.step(preparation_grant(1, 4096)).unwrap(), ChildEmitPreparationStep::Ready(_)) { ready = true; break; }
    }
    assert!(ready);
    assert_eq!(preparation.retained_operation_count(), 3);
    assert!(preparation.take_ready().is_none());
    assert!(preparation.take_ready_owned(RetainedCloneGrant { maximum_capacity_bytes: 0, ..grant }).unwrap().is_none());
    assert_eq!(preparation.retained_operation_count(), 3);
    let (mut wire, mut batch, birth) = preparation.take_ready_owned(grant).unwrap().unwrap();
    assert!(birth.fits(grant));
    assert_eq!(batch.mutations::<TestMutation>().unwrap().as_ptr(), pointer);
    assert_eq!(batch.mutations::<TestMutation>().unwrap().len(), 3);
    for (index, operation) in batch.mutations::<TestMutation>().unwrap().iter().enumerate() {
        match operation { TestMutation::SetCount(value) => assert_eq!(value.value as i64, fixture["values"][index].as_i64().unwrap()), _ => panic!("original typed source variant changed") }
    }
    assert!(wire.ops.is_empty() && wire.labels.is_empty());
    assert_eq!(Value::from(semio_framework_value::ToValue::to_value(&wire)), serde_json::to_value(&wire).unwrap());
    assert!(preparation.terminal_is_empty());
    for _ in 0..1000 {
        if batch.terminal_is_empty() { break; }
        assert!(batch.close_granted(grant).unwrap().progress().fits(grant));
    }
    assert!(batch.terminal_is_empty());
    for _ in 0..1000 {
        let bytes = wire.retirement_demands().unwrap().release_bytes;
        if matches!(wire.close_one(child_release_grant(bytes)).unwrap(), store::RetainedCloneStep::Complete(_)) { break; }
    }
    assert_eq!(wire.retirement_demands().unwrap().release_bytes, 0);
}

#[test]
fn child_emission_preview_retains_exact_encoded_operations_and_semantic_labels() {
    use semio_framework_plugin_decl_emission::child_emit_preparation::{ChildEmitPreparation, ChildEmitPreparationStep};
    let fixture: Value = serde_json::from_str(include_str!("../../../../🏪️store/🧩️composition/📨️emission/📦️owned/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let operations = fixture["values"].as_array().unwrap().iter().map(|value| TestMutation::SetCount(SetCount { value: value.as_i64().unwrap() as i32 })).collect::<Vec<_>>();
    let expected_ops = operations.iter().map(|operation| ::protocol::OpBinary::encode_op(operation).unwrap()).collect::<Vec<_>>();
    let expected_labels = operations.iter().map(|operation| protocol::SemanticMutation::<TestSnapshot>::label(operation)).collect::<Vec<_>>();
    let mut preparation = ChildEmitPreparation::of::<TestSnapshot, _>("fixture", "child", operations);
    let mut ready = false;
    for _ in 0..1000 {
        if matches!(preparation.step(preparation_grant(1, 4096)).unwrap(), ChildEmitPreparationStep::Ready(_)) { ready = true; break; }
    }
    assert!(ready);
    let mut wire = preparation.take_ready().unwrap();
    assert_eq!(wire.ops, expected_ops);
    assert_eq!(wire.labels, expected_labels);
    assert_eq!(Value::from(semio_framework_value::ToValue::to_value(&wire)), serde_json::to_value(&wire).unwrap());
    assert!(preparation.terminal_is_empty());
    for _ in 0..1000 {
        let demand = wire.retirement_demands().unwrap().release_bytes;
        if matches!(wire.close_one(child_release_grant(demand)).unwrap(), store::RetainedCloneStep::Complete(_)) { break; }
    }
    assert_eq!(wire.retirement_demands().unwrap().release_bytes, 0);
}

#[derive(Clone, ToValue, FromValue, semio_framework_value::RetireOwned)]
struct RefusingChildOperation {
    pub inner: TestMutation,
    pub refuse: bool,
}

#[test]
fn child_emission_owned_apply_admits_typed_source_without_requesting_wire_codec() {
    use semio_framework_plugin_decl_emission::child_emit_preparation::{ChildEmitPreparation, ChildEmitPreparationStep};
    use semio_framework_value::retained_clone::RetainedCloneGrant;
    let fixture: Value = serde_json::from_str(include_str!("../../../../🏪️store/🧩️composition/📨️emission/📦️owned/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let operations = fixture["values"].as_array().unwrap().iter().map(|value| RefusingChildOperation { inner: TestMutation::SetCount(SetCount { value: value.as_i64().unwrap() as i32 }), refuse: true }).collect::<Vec<_>>();
    let pointer = operations.as_ptr();
    let mut preparation = ChildEmitPreparation::of_owned::<TestSnapshot, _>("fixture", "child", operations);
    let mut ready = false;
    for _ in 0..100 {
        match preparation.step(preparation_grant(1, 4096)).unwrap() {
            ChildEmitPreparationStep::Ready(_) => { ready = true; break; },
            ChildEmitPreparationStep::Pending(_) => {},
            ChildEmitPreparationStep::Refused(fault,_) => panic!("applying typed admission must never request unused wire codec: {}", fault.message),
        }
    }
    assert!(ready);
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 64, maximum_capacity_bytes: 4096, maximum_release_bytes: 4096, maximum_depth: 64 };
    let (mut metadata, mut batch, progress) = preparation.take_ready_owned(grant).unwrap().unwrap();
    assert!(progress.fits(grant));
    let retained = batch.mutations::<RefusingChildOperation>().unwrap();
    assert_eq!(retained.as_ptr(), pointer);
    assert_eq!(retained.len(), fixture["values"].as_array().unwrap().len());
    assert!(retained.iter().all(|operation| operation.refuse));
    assert!(metadata.ops.is_empty());
    assert_eq!(Value::from(semio_framework_value::ToValue::to_value(&metadata)), serde_json::to_value(&metadata).unwrap());
    for _ in 0..1000 {
        if batch.terminal_is_empty() { break; }
        assert!(batch.close_granted(grant).unwrap().progress().fits(grant));
    }
    assert!(batch.terminal_is_empty() && preparation.terminal_is_empty());
    for _ in 0..1000 {
        let demand = metadata.retirement_demands().unwrap().release_bytes;
        if matches!(metadata.close_one(child_release_grant(demand)).unwrap(), store::RetainedCloneStep::Complete(_)) { break; }
    }
    assert_eq!(metadata.retirement_demands().unwrap().release_bytes, 0);
}

impl protocol::Mutation<TestSnapshot> for RefusingChildOperation {
    type Diff = <TestMutation as protocol::Mutation<TestSnapshot>>::Diff;
    const DESCRIPTORS: &'static [::protocol::MutationLeafDescriptor] = <TestMutation as protocol::Mutation<TestSnapshot>>::DESCRIPTORS;
    fn descriptor(&self) -> &'static ::protocol::MutationLeafDescriptor { protocol::Mutation::descriptor(&self.inner) }


    fn retire_cold(self) { protocol::Mutation::<TestSnapshot>::retire_cold(self.inner); }
}

impl protocol::ColdMutation<TestSnapshot> for RefusingChildOperation {
    type ReferenceDiff = <TestMutation as protocol::Mutation<TestSnapshot>>::Diff;
    fn diff(&self, base: &TestSnapshot) -> ::protocol::MutationOutcome<Self::ReferenceDiff> { protocol::ColdMutation::diff(&self.inner, base) }
    fn inverse(&self, base: &TestSnapshot) -> Result<Vec<Self>, semio_framework_value::ValueError> { Ok(protocol::ColdMutation::inverse(&self.inner, base)?.into_iter().map(|inner| Self { inner, refuse: self.refuse }).collect()) }
}

impl protocol::SemanticMutation<TestSnapshot> for RefusingChildOperation {
    fn kinds() -> &'static [protocol::SemanticDescriptor] { <TestMutation as protocol::SemanticMutation<TestSnapshot>>::kinds() }
    fn semantics(&self) -> &'static protocol::SemanticDescriptor { protocol::SemanticMutation::semantics(&self.inner) }
    fn label(&self) -> LocalizedLabel { protocol::SemanticMutation::label(&self.inner) }
    fn target(&self) -> Vec<String> { protocol::SemanticMutation::target(&self.inner) }
}

impl ::protocol::OpBinary for RefusingChildOperation {
    fn encode_op(&self) -> Result<Vec<u8>, ::protocol::ProtocolError> {
        if self.refuse { return Err(::protocol::ProtocolError::Malformed { what: "owned-child-operation", offset: 9, detail: "codec refusal \0引用😀".to_string() }); }
        ::protocol::OpBinary::encode_op(&self.inner)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, ::protocol::ProtocolError> { Ok(Self { inner: <TestMutation as ::protocol::OpBinary>::decode_op(bytes)?, refuse: false }) }
}

#[test]
fn child_emission_encoder_refusal_preserves_prior_complete_owned_operations() {
    let fixture: Value = serde_json::from_str(include_str!("../../../🧩️composition/📨️emission/🧪️tests/🧫️fixtures/🔣️.json")).expect("closed emission contract");
    let operation = RefusingChildOperation { inner: TestMutation::SetCount(SetCount { value: fixture["validValue"].as_i64().expect("i32") as i32 }), refuse: false };
    let mut emit = ChildEmit::open(fixture["slot"].as_str().expect("literal slot"), fixture["childId"].as_str().expect("literal child"), 2);
    let _ = emit.push::<TestSnapshot, _>(&operation);
    assert_eq!(emit.ops, vec![::protocol::OpBinary::encode_op(&operation).expect("actual complete operation")]);
    let accepted_ops = emit.ops.clone();
    let accepted_labels = emit.labels.clone();
    let accepted_schema = emit.op_schema.clone();
    let refusing = RefusingChildOperation { inner: operation.inner.clone(), refuse: true };
    let _ = emit.push::<TestSnapshot, _>(&refusing);
    assert_eq!(emit.ops, accepted_ops, "a codec refusal cannot publish an empty substitute operation");
    assert_eq!(emit.labels, accepted_labels, "the refused operation has no accepted label");
    assert_eq!(emit.op_schema, accepted_schema);
    assert_eq!(emit.ops.len(), fixture["expected"]["acceptedOperations"].as_u64().expect("count") as usize);
    assert_eq!(emit.labels.len(), fixture["expected"]["acceptedLabels"].as_u64().expect("count") as usize);
    <RefusingChildOperation as protocol::Mutation<TestSnapshot>>::retire_cold(operation);
    <RefusingChildOperation as protocol::Mutation<TestSnapshot>>::retire_cold(refusing);
}

#[derive(Clone)]
struct TrackedChildOperation {
    pub inner:RefusingChildOperation,
    pub payload:Vec<u8>,
    pub retired:bool,
    pub returned:std::sync::Arc<std::sync::atomic::AtomicUsize>,
}
impl semio_framework_value::ToValue for TrackedChildOperation{
    fn to_value(&self)->semio_framework_value::DslValue{semio_framework_value::DslValue::Object(vec![("inner".into(),semio_framework_value::ToValue::to_value(&self.inner)),("payload".into(),semio_framework_value::ToValue::to_value(&self.payload))])}
}
impl semio_framework_value::FromValue for TrackedChildOperation{
    fn from_value(_:semio_framework_value::DslValue)->Result<Self,semio_framework_value::ValueError>{Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"tracked emission source requires its registered owning fixture issuer"))}
}
impl Drop for TrackedChildOperation{
    fn drop(&mut self){assert!(std::thread::panicking()||(self.retired&&self.payload.capacity()==0),"actual typed child operation payload lost before its genuine retirement");}
}
impl ::protocol::OpBinary for TrackedChildOperation{
    fn encode_op(&self)->Result<Vec<u8>,::protocol::ProtocolError>{self.inner.encode_op()}
    fn decode_op(_: &[u8])->Result<Self,::protocol::ProtocolError>{Err(::protocol::ProtocolError::LimitExceeded("test source is issued only by the owned fixture"))}
}
impl protocol::Mutation<TestSnapshot> for TrackedChildOperation{
    type Diff=<RefusingChildOperation as protocol::Mutation<TestSnapshot>>::Diff;
    const DESCRIPTORS:&'static[::protocol::MutationLeafDescriptor]=<RefusingChildOperation as protocol::Mutation<TestSnapshot>>::DESCRIPTORS;
    fn descriptor(&self)->&'static ::protocol::MutationLeafDescriptor{protocol::Mutation::descriptor(&self.inner)}


    fn retire_cold(self){panic!("actual owned emission must use the installed retirement continuation, not Mutation::retire_cold");}
}

impl protocol::ColdMutation<TestSnapshot> for TrackedChildOperation{
    type ReferenceDiff = <RefusingChildOperation as protocol::Mutation<TestSnapshot>>::Diff;
    fn diff(&self,base:&TestSnapshot)->::protocol::MutationOutcome<Self::ReferenceDiff>{protocol::ColdMutation::diff(&self.inner,base)}
    fn inverse(&self,_:&TestSnapshot)->Result<Vec<Self>,semio_framework_value::ValueError>{Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"tracked fixture mutation has no inverse source admission"))}
}
impl protocol::SemanticMutation<TestSnapshot> for TrackedChildOperation{
    fn kinds()->&'static[protocol::SemanticDescriptor]{<RefusingChildOperation as protocol::SemanticMutation<TestSnapshot>>::kinds()}
    fn semantics(&self)->&'static protocol::SemanticDescriptor{protocol::SemanticMutation::semantics(&self.inner)}
    fn label(&self)->LocalizedLabel{protocol::SemanticMutation::label(&self.inner)}
    fn target(&self)->Vec<String>{protocol::SemanticMutation::target(&self.inner)}
}
struct TrackedChildRetirement{pub operation:Option<TrackedChildOperation>,refuse_once:bool}
impl store::ErasedSnapshotRetirement for TrackedChildRetirement{
 fn next_demand(&self,maximum_body_bytes:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{let _=maximum_body_bytes;Ok(semio_framework_value::RetirementDemand{release_bytes:self.operation.as_ref().map_or(0,|operation|operation.payload.capacity()),depth:usize::from(self.operation.is_some()),..Default::default()})}

    fn close_step(&mut self,grant:store::RetainedCloneGrant)->Result<store::RetainedCloneStep,semio_framework_value::ValueError>{
        let empty=store::RetainedCloneProgress::default();
        if grant.maximum_items==0||grant.maximum_depth==0{return Ok(store::RetainedCloneStep::Progress(empty));}
        if self.refuse_once{self.refuse_once=false;return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"actual accepted mutation retirement refusal"));}
        let Some(operation)=self.operation.as_mut()else{return Ok(store::RetainedCloneStep::Complete(empty))};
        let allocation=operation.payload.capacity();
        if allocation>grant.maximum_release_bytes{return Ok(store::RetainedCloneStep::Progress(empty));}
        if allocation!=0{operation.payload=Vec::new();return Ok(store::RetainedCloneStep::Progress(store::RetainedCloneProgress{copied_items:1,released_bytes:allocation,..empty}));}
        operation.retired=true;operation.returned.fetch_add(1,std::sync::atomic::Ordering::SeqCst);self.operation.take();
        Ok(store::RetainedCloneStep::Complete(store::RetainedCloneProgress{copied_items:1,..empty}))
    }
    fn terminal_is_empty(&self)->bool{self.operation.is_none()}
    fn next_copy_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_capacity_byte_demand(&self,_:usize)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_release_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(self.operation.as_ref().map_or(0,|operation|operation.payload.capacity()))}
    fn next_depth_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(usize::from(self.operation.is_some()))}
}
#[derive(semio_framework_value::FactoryPayloadRetirement)]
struct TrackedChildRetirementFactory;
impl store::ArtifactOwnedValueRetirementFactory<TrackedChildOperation> for TrackedChildRetirementFactory{
    fn retirement_birth_bytes(&self,_:&TrackedChildOperation)->usize{std::mem::size_of::<TrackedChildRetirement>()}
    fn retire_owned(&self,operation:TrackedChildOperation,grant:store::RetainedCloneGrant)->Result<(Box<dyn store::ErasedSnapshotRetirement>,store::RetainedCloneProgress),(semio_framework_value::ValueError,TrackedChildOperation)>{
        semio_framework_value::retirement::frame::admit_retirement_frame(operation,grant,|operation|TrackedChildRetirement{operation:Some(operation),refuse_once:false})
    }
}
#[derive(semio_framework_value::FactoryPayloadRetirement)]
struct RefusingTrackedChildRetirementFactory;
impl store::ArtifactOwnedValueRetirementFactory<TrackedChildOperation> for RefusingTrackedChildRetirementFactory{
    fn retirement_birth_bytes(&self,_:&TrackedChildOperation)->usize{std::mem::size_of::<TrackedChildRetirement>()}
    fn retire_owned(&self,operation:TrackedChildOperation,grant:store::RetainedCloneGrant)->Result<(Box<dyn store::ErasedSnapshotRetirement>,store::RetainedCloneProgress),(semio_framework_value::ValueError,TrackedChildOperation)>{
        semio_framework_value::retirement::frame::admit_retirement_frame(operation,grant,|operation|TrackedChildRetirement{operation:Some(operation),refuse_once:true})
    }
}

#[test]
fn child_emission_owned_refusal_keeps_prefix_rejected_and_remaining_until_actual_typed_retirement(){
    use semio_framework_plugin_decl_emission::child_emit_preparation::{ChildEmitPreparation, ChildEmitPreparationStep};
    use semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep;
    let fixture:Value=serde_json::from_str(include_str!("../../../🧩️composition/📨️emission/🧪️tests/🧫️fixtures/🔣️.json")).expect("closed owned emission law");
    let demand=&fixture["ownedPreparation"];
    let returned=std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let operations=demand["operations"].as_array().expect("complete authored source").iter().map(|row|TrackedChildOperation{
        inner:RefusingChildOperation{inner:TestMutation::SetCount(SetCount{value:i32::try_from(row["value"].as_i64().expect("signed source word")).expect("exact i32 word")}),refuse:row["refuse"].as_bool().expect("authored refusal")},
        payload:row["payload"].as_array().expect("actual owned source payload").iter().map(|word|u8::try_from(word.as_u64().expect("unsigned byte")).expect("exact byte")).collect(),
        retired:false,returned:std::sync::Arc::clone(&returned),
    }).collect::<Vec<_>>();
    let count=operations.len();
    let factory:std::sync::Arc<dyn store::ArtifactOwnedValueRetirementFactory<TrackedChildOperation>>=std::sync::Arc::new(TrackedChildRetirementFactory);
    let mut preparation=ChildEmitPreparation::with_factory::<TestSnapshot,_>(fixture["slot"].as_str().unwrap().to_owned(),fixture["childId"].as_str().unwrap().to_owned(),operations,std::sync::Arc::clone(&factory));
    assert!(matches!(preparation.step(preparation_grant(1, 0)).expect("zero-byte retained step"),ChildEmitPreparationStep::Pending(_)));
    assert_eq!(preparation.retained_operation_count(),count);
    assert_eq!(returned.load(std::sync::atomic::Ordering::SeqCst),0);
    let maximum=demand["maximumAllocationGrant"].as_u64().unwrap() as usize;
    let mut refused=false;
    for _ in 0..demand["maximumPumpSteps"].as_u64().unwrap(){
        match preparation.step(preparation_grant(1, maximum)).expect("actual owned producer"){
            ChildEmitPreparationStep::Pending(_)=>{},
            ChildEmitPreparationStep::Refused(fault,_)=>{assert_eq!(fault.code.0,"module.protocol");refused=true;break;},
            ChildEmitPreparationStep::Ready(_)=>panic!("closed second encoder refusal cannot issue a complete group"),
        }
    }
    assert!(refused);
    let prefix=preparation.accepted_prefix().expect("actual retained prefix");
    assert_eq!(prefix.slot,fixture["slot"].as_str().unwrap());assert_eq!(prefix.child_id,fixture["childId"].as_str().unwrap());
    assert_eq!(prefix.ops.len(),demand["expected"]["acceptedOperations"].as_u64().unwrap() as usize);
    let independent=serde_json::to_value(prefix).expect("independent complete wire projection");
    assert_eq!(Value::from(semio_framework_value::ToValue::to_value(prefix)),independent);
    assert!(matches!(preparation.refusal(),Some(::protocol::ProtocolError::Malformed{what:"owned-child-operation",offset:9,detail})if detail==fixture["refusal"]["detail"].as_str().unwrap()));
    assert_eq!(returned.load(std::sync::atomic::Ordering::SeqCst),demand["expected"]["retiredBeforeRefusal"].as_u64().unwrap() as usize);
    assert_eq!(preparation.retained_operation_count(),demand["expected"]["retainedAfterRefusal"].as_u64().unwrap() as usize);
    preparation.begin_close();
    assert_eq!(preparation.close_step(child_zero_grant()).expect("zero close"),PluginLifecycleStep::Progress(Default::default()));
    assert_eq!(preparation.retained_operation_count(),demand["expected"]["retainedAfterRefusal"].as_u64().unwrap() as usize);
    assert_eq!(returned.load(std::sync::atomic::Ordering::SeqCst),demand["expected"]["retiredAfterZeroGrant"].as_u64().unwrap() as usize);
    assert_eq!(preparation.close_step(store::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:1,maximum_capacity_bytes:1,maximum_release_bytes:1,maximum_depth:64}).expect("unfunded typed owner close"),PluginLifecycleStep::Progress(Default::default()));
    assert_eq!(preparation.retained_operation_count(),2);
    let original_close_grant=preparation_grant(1,maximum);
    let mut returned_provider=None;
    for _ in 0..demand["maximumCloseSteps"].as_u64().unwrap(){
        if preparation.terminal_is_empty(){break;}
        let grant=original_close_grant;assert!(semio_framework_plugin_ownership::bounded_ownership::plugin_grant_funds(preparation.retirement_demands(grant.maximum_copy_bytes).expect("quoted retained close"),grant));
        let step=preparation.close_step(grant).expect("actual retained owned close");
        if let PluginLifecycleStep::Progress(progress)=step{assert!(progress.fits(grant));}
        else if let PluginLifecycleStep::AwaitingInput{reason}=step{
            assert_eq!(reason,"custom child emission retirement provider awaits its owning caller handback");
            assert!(returned_provider.is_none());
            let provider=preparation.take_retirement_provider::<TrackedChildOperation>().expect("same typed provider owner returned");
            assert!(std::sync::Arc::ptr_eq(&provider,&factory));returned_provider=Some(provider);
        }else{assert!(matches!(step,PluginLifecycleStep::Complete(progress) if progress.fits(grant)));}
    }
    assert!(preparation.terminal_is_empty());
    assert_eq!(returned_provider.is_some(),demand["expected"]["providerReturnedToCaller"].as_bool().unwrap());
    assert_eq!(returned.load(std::sync::atomic::Ordering::SeqCst),demand["expected"]["retiredAfterClose"].as_u64().unwrap() as usize);
}

#[test]
fn child_emission_accepted_owner_retirement_refusal_retains_exact_prefix_and_provider(){
    use semio_framework_plugin_decl_emission::child_emit_preparation::{ChildEmitPreparation, ChildEmitPreparationStep};
    use semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep;
    let fixture:Value=serde_json::from_str(include_str!("../../../🧩️composition/📨️emission/🧪️tests/🧫️fixtures/🔣️.json")).expect("closed accepted retirement refusal");
    let demand=&fixture["ownedPreparation"];
    let returned=std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let operation=TrackedChildOperation{
        inner:RefusingChildOperation{inner:TestMutation::SetCount(SetCount{value:fixture["validValue"].as_i64().unwrap() as i32}),refuse:false},
        payload:demand["operations"][0]["payload"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap() as u8).collect(),
        retired:false,returned:std::sync::Arc::clone(&returned),
    };
    let factory:std::sync::Arc<dyn store::ArtifactOwnedValueRetirementFactory<TrackedChildOperation>>=std::sync::Arc::new(RefusingTrackedChildRetirementFactory);
    let mut preparation=ChildEmitPreparation::with_factory::<TestSnapshot,_>(fixture["slot"].as_str().unwrap().into(),fixture["childId"].as_str().unwrap().into(),vec![operation],std::sync::Arc::clone(&factory));
    let maximum=demand["maximumAllocationGrant"].as_u64().unwrap() as usize;
    let mut refused=false;
    for _ in 0..demand["maximumPumpSteps"].as_u64().unwrap(){
        match preparation.step(preparation_grant(1, maximum)){Ok(ChildEmitPreparationStep::Pending(_))=>{},Err(fault)=>{assert_eq!(fault.code.0,"interactive-job.child-emission-retirement-refused");refused=true;break},_=>panic!("accepted owner retirement refusal cannot produce a complete group")}
    }
    assert!(refused);
    let error=preparation.retirement_refusal().expect("original typed retirement error");
    assert_eq!(error.kind,semio_framework_value::ValueRefusalKind::UnsupportedOwner);
    assert_eq!(error.message,demand["retirementRefusal"]["message"].as_str().unwrap());
    assert_eq!(preparation.retained_operation_count(),1);
    let prefix=preparation.accepted_prefix().unwrap();
    assert_eq!(prefix.ops.len(),1);assert_eq!(prefix.labels.len(),1);
    assert_eq!(Value::from(semio_framework_value::ToValue::to_value(prefix)),serde_json::to_value(prefix).unwrap());
    assert_eq!(returned.load(std::sync::atomic::Ordering::SeqCst),0);
    preparation.begin_close();
    assert_eq!(preparation.close_step(child_zero_grant()).unwrap(),PluginLifecycleStep::Progress(Default::default()));
    assert!(preparation.retirement_refusal().is_some());
    let original_close_grant=preparation_grant(1,maximum);
    let mut provider_returned=false;
    for _ in 0..demand["maximumCloseSteps"].as_u64().unwrap(){
        if preparation.terminal_is_empty(){break;}
        let grant=original_close_grant;assert!(semio_framework_plugin_ownership::bounded_ownership::plugin_grant_funds(preparation.retirement_demands(grant.maximum_copy_bytes).expect("quoted accepted owner close"),grant));
        match preparation.close_step(grant).expect("retained accepted owner close"){
            PluginLifecycleStep::AwaitingInput{..}=>{let provider=preparation.take_retirement_provider::<TrackedChildOperation>().expect("original retained provider handback");assert!(std::sync::Arc::ptr_eq(&provider,&factory));provider_returned=true;},
            PluginLifecycleStep::Progress(progress)=>{assert!(progress.fits(grant));},
            PluginLifecycleStep::Complete(_)=>{},
            _=>panic!("genuine fixture provider closes after its one refusal"),
        }
    }
    assert!(provider_returned&&preparation.terminal_is_empty());
    assert_eq!(returned.load(std::sync::atomic::Ordering::SeqCst),1);
}

#[test]
fn ready_child_parent_return_keeps_nested_owner_backing_until_paid_parent_release(){
    use semio_framework_value::retirement::allocation_return::{ParentAllocationReturn,AllocationReturnStep};
    use semio_framework_plugin_decl_emission::ChildEmit;
    use semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🧩️composition/📨️emission/🧪️tests/🧫️fixtures/📦️nested-parent-return.json")).unwrap();
    let mut child=ChildEmit::open(fixture["slot"].as_str().unwrap(),fixture["childId"].as_str().unwrap(),0);child.owner=fixture["owner"].as_str().unwrap().to_owned();
    let owner_pointer=child.owner.as_ptr();let expected=child.owner.capacity()+child.slot.capacity()+child.child_id.capacity()+child.op_schema.0.capacity();
    let mut emit:Emit<TestMutation,NoConfigMutation,NoDraftMutation>=Emit::default();emit.child_emits.push(child);let expected=expected+emit.child_emits.capacity()*std::mem::size_of::<ChildEmit>();
    let items=fixture["maximumItems"].as_u64().unwrap()as usize;let child_bytes=fixture["maximumChildBytes"].as_u64().unwrap()as usize;let parent_bytes=fixture["maximumParentBytes"].as_u64().unwrap()as usize;
    let mut parent=ParentAllocationReturn::<16>::try_new(parent_bytes,parent_bytes*fixture["parentSlots"].as_u64().unwrap()as usize).unwrap();
    let return_grant=|items:usize|store::RetainedCloneGrant{maximum_items:items,maximum_release_bytes:child_bytes,maximum_depth:64,..Default::default()};
    assert_eq!(emit.return_child_one(&mut parent,return_grant(0)).unwrap(),Some(PluginLifecycleStep::Progress(Default::default())));assert_eq!(emit.child_emits[0].owner.as_ptr(),owner_pointer);assert_eq!(parent.retained_bytes(),0);
    let mut complete=false;for _ in 0..fixture["maximumTurns"].as_u64().unwrap(){match emit.return_child_one(&mut parent,return_grant(items)).unwrap(){None=>{complete=true;break;},Some(PluginLifecycleStep::Progress(progress))=>{assert!(progress.copied_items<=items);assert_eq!(progress.released_bytes,0);},other=>panic!("child handoff issues logical pending or empty recipient lane: {other:?}")}}
    assert!(complete);assert!(emit.child_emits.is_empty());assert_eq!(emit.child_emits.capacity(),0);assert_eq!(parent.retained_bytes(),expected,"every original child field and backing allocation remains physically owned by the actual parent");assert!(!parent.terminal_is_empty());let before=parent.retained_bytes();assert_eq!(parent.close_step(items,child_bytes),AllocationReturnStep::Pending{released_items:0,released_bytes:0});assert_eq!(parent.retained_bytes(),before);
    let mut released=0;for _ in 0..fixture["maximumTurns"].as_u64().unwrap(){match parent.close_step(items,parent_bytes){AllocationReturnStep::Complete=>break,AllocationReturnStep::Pending{released_items,released_bytes}=>{assert!(released_items<=items);assert!(released_bytes<=parent_bytes);released+=released_bytes;}}}assert!(parent.terminal_is_empty());assert_eq!(parent.retained_bytes(),0);assert_eq!(released,expected);
}

#[test]
fn child_emit_original_full_grant_preserves_denied_pointers_and_system_release_receipts() {
    use semio_framework_value::retained_clone::{RetainedCloneGrant,RetainedCloneProgress};
    let fixture:Value=serde_json::from_str(include_str!("../../../🧵️retained-command/🧪️tests/🧫️fixtures/🧩️child-prepublication-close.json")).unwrap();
    let policy=&fixture["closeGrant"];let axis=|name:&str|policy[name].as_u64().unwrap() as usize;
    let grant=RetainedCloneGrant{maximum_items:axis("maximumItems"),maximum_copy_bytes:axis("maximumCopyBytes"),maximum_capacity_bytes:axis("maximumCapacityBytes"),maximum_release_bytes:axis("maximumReleaseBytes"),maximum_depth:axis("maximumDepth")};
    for row in fixture["children"].as_array().unwrap(){
        let(mut child,birth)=semio_framework_trace::observe_heap_allocations_on_this_thread(||semio_framework_plugin_decl_emission::ChildEmit{genesis:None,owner:row["id"].as_str().unwrap().into(),slot:row["slot"].as_str().unwrap().into(),child_id:row["childId"].as_str().unwrap().into(),ops:vec![row["value"].as_str().unwrap().as_bytes().to_vec()],op_schema:semio_framework::kernel::SchemaId("child.current".into()),labels:vec![semio_framework_ui_locale::LocalizedLabel::data(row["value"].as_str().unwrap().to_owned())]});
        let owner=child.owner.as_ptr();let operations=child.ops.as_ptr();let first=child.ops[0].as_ptr();
        let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||child.close_one(RetainedCloneGrant{maximum_items:0,..grant}).unwrap());
        assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!((child.owner.as_ptr(),child.ops.as_ptr(),child.ops[0].as_ptr()),(owner,operations,first));
        let mut released=0;
        for _ in 0..1024{
            if child.terminal_is_empty(){break;}
            let demand=child.retirement_demands().unwrap();assert!(demand.release_bytes<=grant.maximum_release_bytes);assert!(demand.depth<=grant.maximum_depth);
            if demand.release_bytes!=0{
                let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||child.close_one(RetainedCloneGrant{maximum_release_bytes:demand.release_bytes-1,..grant}).unwrap());
                assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
            }
            let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||child.close_one(grant).unwrap());let progress=step.progress();assert!(progress.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));released+=progress.released_bytes;
        }
        assert!(child.terminal_is_empty());assert_eq!(released,birth.requested_bytes);let((),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(child));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
    }
}
