//! 🧪️ Actual fixed-page query output, exact ACK authority, cancellation, and Store read return.

use super::*;
use crate::local_interaction::retirement::interaction_store_owners;
use crate::app::InteractionConfigMutation;
use protocol::InteractionState;
use semio_framework_value::retained_clone::RetainedCloneBirthDemand;
use store::{ArtifactStore, ErasedSnapshotRetirement, SpaceMember};

type InteractionStore = ArtifactStore<InteractionState, InteractionConfigMutation>;

#[semio_framework_async_macros::async_test]
async fn original_query_output_width_does_not_price_or_renew_its_caller_receiving_grant(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/📨️authority.json")).unwrap();let original:RetainedCloneGrant=serde_json::from_value(law["wholeOperationGrant"].clone()).unwrap();
 for width in law["outputWidths"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap()as usize){
  let(mut store,mut query,expected,mut recipient)=fixture(law["sourceCase"].as_str().unwrap()).await;let mut output=Vec::new();let mut zero=false;let mut terminal=false;
  for _ in 0..law["maximumTurns"].as_u64().unwrap(){
   let grant=ArtifactStoreOneItemGrant::from_retained(recipient.remaining_grant());let(step,physical)=semio_framework_trace::observe_heap_allocations_on_this_thread(||query.advance(grant,width).unwrap());let progress=step.progress();recipient.record_progress(progress).unwrap();assert_eq!((physical.requested_bytes,physical.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));assert!(recipient.progress().fits(original));if let LocalInteractionQueryStep::Advanced{emitted_bytes:0,retired_bytes:0,..}=step{zero|=progress!=Default::default()}
   if let Some(page)=query.page(){assert!(page.bytes.len()<=width.min(LOCAL_INTERACTION_QUERY_PAGE_BYTES));output.extend_from_slice(page.bytes);terminal=page.terminal;let token=page.token.clone();assert!(query.acknowledge(&token));if terminal{break}}
  }
  assert!(terminal);assert_eq!(output,expected);assert_eq!(serde_json::to_value(zero).unwrap(),law["zeroByteProgressRetained"]);close_original_query(&mut store,&mut query,&mut recipient,law["maximumTurns"].as_u64().unwrap());assert!(recipient.progress().fits(original));eprintln!("[DEBUG] Original query outputWidth={width} distinctCallerGrant=true cumulativeRecipient=true zeroByteOwnership=true originalReadReturned=true physicalReceipts=true independentSerde=true");
 }
}
fn close_original_query(store:&mut InteractionStore,query:&mut LocalInteractionQuery,recipient:&mut store::NativeSnapshotBodyWallet,turns:u64){
 let mut owner:Option<Box<dyn ErasedSnapshotRetirement>>=None;
 for _ in 0..turns{
  let(step,physical)=semio_framework_trace::observe_heap_allocations_on_this_thread(||query.close_step(recipient.remaining_grant()).unwrap());let progress=step.progress();recipient.record_progress(progress).unwrap();assert_eq!((physical.requested_bytes,physical.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));
  if owner.is_none(){let((received,progress),physical)=semio_framework_trace::observe_heap_allocations_on_this_thread(||store.take_returned_snapshot_read_retirement(recipient.remaining_grant()).unwrap());recipient.record_progress(progress).unwrap();assert_eq!((physical.requested_bytes,physical.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));owner=received;}
  if let Some(active)=owner.as_mut(){let(step,physical)=semio_framework_trace::observe_heap_allocations_on_this_thread(||active.close_step(recipient.remaining_grant()).unwrap());let progress=step.progress();recipient.record_progress(progress).unwrap();assert_eq!((physical.requested_bytes,physical.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));if matches!(step,RetainedCloneStep::Complete(_)){assert!(active.terminal_is_empty());let(_,physical)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(owner.take()));assert_eq!((physical.requested_bytes,physical.released_bytes),(0,0));}}
  if query.terminal_is_empty()&&owner.is_none()&&store.snapshot_read_leases_terminal_is_empty(){break}
 }
 assert!(query.terminal_is_empty());assert!(owner.is_none());assert!(store.snapshot_read_leases_terminal_is_empty());assert_eq!(query.completed_bytes(),query.retired_bytes());
 for _ in 0..turns{let(step,physical)=semio_framework_trace::observe_heap_allocations_on_this_thread(||store.close_owned_step(recipient.remaining_grant()).unwrap());let progress=step.progress();recipient.record_progress(progress).unwrap();assert_eq!((physical.requested_bytes,physical.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));if matches!(step,RetainedCloneStep::Complete(_)){assert!(store.close_owned_terminal_is_empty());return}}
 panic!("original query caller grant did not close its Store");
}

pub(crate) fn original_receiving_recipient() -> store::NativeSnapshotBodyWallet {
    let law:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/📨️authority.json")).unwrap();
    store::NativeSnapshotBodyWallet::new(serde_json::from_value(law["wholeOperationGrant"].clone()).unwrap())
}

pub(crate) fn install_original_interaction_owners(store:&mut InteractionStore,recipient:&mut store::NativeSnapshotBodyWallet){
    let(result,physical)=semio_framework_trace::observe_heap_allocations_on_this_thread(||interaction_store_owners(recipient.remaining_grant()));
    let(mut owners,progress)=match result{Ok(received)=>received,Err(error)=>panic!("original interaction catalog refused: {}",error.error)};
    recipient.record_progress(progress).unwrap();assert_eq!((physical.requested_bytes,physical.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));
    while !owners.constructor_is_complete(){
        let(result,physical)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owners.admit_constructor(recipient.remaining_grant()));let progress=result.unwrap_or_else(|(error,_)|panic!("original interaction catalog ticket refused: {error}"));recipient.record_progress(progress).unwrap();assert_eq!((physical.requested_bytes,physical.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));assert_ne!(progress,Default::default());
    }
    store.install_document_store_owners_exact(owners).unwrap_or_else(|(error,_)|panic!("original interaction catalog installation refused: {error}"));
}

#[semio_framework_async_macros::async_test]
async fn original_local_interaction_copy_one_refuses_metadata_close_and_preserves_owner(){
    let law:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/📨️authority.json")).unwrap();
    let original:RetainedCloneGrant=serde_json::from_value(law["originalGrants"][0].clone()).unwrap();
    assert_eq!(original.maximum_copy_bytes,1);
    let(mut store,mut query,_,mut recipient)=fixture(law["sourceCase"].as_str().unwrap()).await;
    query.cancel();let token=query.token().clone();let demand=query.retirement_demands(original.maximum_copy_bytes).unwrap();
    let(step,physical)=semio_framework_trace::observe_heap_allocations_on_this_thread(||query.close_step(original).unwrap());
    assert_eq!(step.progress(),Default::default());assert_eq!((physical.requested_bytes,physical.released_bytes),(0,0));assert!(!query.terminal_is_empty());assert_eq!(query.token(),&token);assert_eq!(query.retirement_demands(original.maximum_copy_bytes).unwrap(),demand);assert!(!store.snapshot_read_leases_terminal_is_empty());
    close_original_query(&mut store,&mut query,&mut recipient,law["maximumTurns"].as_u64().unwrap());
    eprintln!("[DEBUG] Original query copy1 metadata close genuinely refused heap0 same captured owner/token/demand/read; separate caller whole-operation authority closes actual physical owner");
}

async fn fixture(source_case: &str) -> (InteractionStore, LocalInteractionQuery, Vec<u8>, store::NativeSnapshotBodyWallet) {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../🔨️modules/📡️replication/📡️wire/🏠️local-interaction/🧫️fixtures/🏠️local-interaction/🔣️.json")).unwrap();
    let row = fixture["cases"].as_array().unwrap().iter().find(|row| row["id"] == source_case).unwrap();
    let mut state = row["expected"].clone();
    state["hover"] = serde_json::json!({"private": {"channel": "pointer", "ids": ["not-captured"]}});
    let state: InteractionState = serde_json::from_value(state).unwrap();
    let envelope = store::create_document_envelope::<InteractionState, InteractionConfigMutation>("framework.interaction", "local-query-test", state, None);
    let mut store = InteractionStore::new(envelope, protocol::ActorId(store::os_spr::LOCAL_ACTOR_ID.into())).await.unwrap();
    let mut recipient=original_receiving_recipient();install_original_interaction_owners(&mut store,&mut recipient);
    let identity = LocalInteractionIdentity { app_instance_id: 7, generation: store.generation_now(), revision: store.content_revision_now(), document_revision: [2; 32], topology_revision: [3; 32] };
    let hex = |bytes: &[u8; 32]| bytes.iter().map(|byte| format!("{byte:02x}")).collect::<String>();
    let expected = serde_json::to_vec(&serde_json::json!({"identity": {
        "appInstanceId": identity.app_instance_id,
        "documentRevision": hex(&identity.document_revision),
        "generation": identity.generation.to_string(),
        "revision": hex(&identity.revision),
        "topologyRevision": hex(&identity.topology_revision),
    }, "state": row["expected"]}))
    .unwrap();
    let capture = LocalInteractionCaptureCursor::new(store.snapshot_read().unwrap(), identity);
    (store, LocalInteractionQuery::new(capture, 13, 41), expected,recipient)
}

fn wrong_token(mut token: LocalInteractionPageToken, field: &str) -> LocalInteractionPageToken {
    match field {
        "request" => token.request_id += 1,
        "queryGeneration" => token.query_generation += 1,
        "ordinal" => token.ordinal += 1,
        "instance" => token.identity.app_instance_id += 1,
        "generation" => token.identity.generation += 1,
        "interaction" => token.identity.revision[31] ^= 1,
        "document" => token.identity.document_revision[31] ^= 1,
        "topology" => token.identity.topology_revision[31] ^= 1,
        _ => panic!("unknown fixture authority field"),
    }
    token
}

#[semio_framework_async_macros::async_test]
async fn local_interaction_query_exact_pages_ack_backpressure_and_terminal_return() {
    let laws: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../🔨️modules/📡️replication/📡️wire/🏠️local-interaction/🧫️fixtures/📃️query/🔣️.json")).unwrap();
    for source in laws["sourceCases"].as_array().unwrap() {
        for bytes in [1, 64, 4096] {
            let (mut store, mut query, expected,mut recipient) = fixture(source.as_str().unwrap()).await;
            let mut actual = Vec::new();
            let mut ordinal = 0;
            for _ in 0..500_000 {
                let grant=ArtifactStoreOneItemGrant::from_retained(recipient.remaining_grant());
                let(step,physical)=semio_framework_trace::observe_heap_allocations_on_this_thread(||query.advance(grant,bytes).unwrap());
                let progress=step.progress();recipient.record_progress(progress).unwrap();assert_eq!((physical.requested_bytes,physical.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));
                let Some(page) = query.page() else { continue };
                assert_eq!(page.token.ordinal, ordinal);
                assert!(page.bytes.len() <= bytes.min(LOCAL_INTERACTION_QUERY_PAGE_BYTES));
                actual.extend_from_slice(page.bytes);
                let terminal = page.terminal;
                let token = page.token.clone();
                let before = query.completed_bytes();
                assert_eq!(query.advance(ArtifactStoreOneItemGrant::from_retained(recipient.remaining_grant()),bytes).unwrap(), LocalInteractionQueryStep::PageReady);
                assert_eq!(query.completed_bytes(), before);
                for field in laws["wrongAcknowledgements"].as_array().unwrap() {
                    assert!(!query.acknowledge(&wrong_token(token.clone(), field.as_str().unwrap())));
                }
                assert!(query.acknowledge(&token));
                assert!(!query.acknowledge(&token));
                assert!(query.page().is_none());
                ordinal += 1;
                if terminal {
                    break;
                }
            }
            assert_eq!(actual, expected);
            assert!(!query.terminal_is_empty());
            close_original_query(&mut store,&mut query,&mut recipient,500_000);
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn local_interaction_query_zero_grants_cancel_and_worker_transfer() {
    let laws: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../🔨️modules/📡️replication/📡️wire/🏠️local-interaction/🧫️fixtures/📃️query/🔣️.json")).unwrap();
    for prefix in laws["cancelAfterBytes"].as_array().unwrap() {
        let (mut store, mut query, _,mut recipient) = fixture("semantic-unicode-over-page").await;let original=recipient.remaining_grant();
        for denied in [RetainedCloneGrant { maximum_items:0,..original }, RetainedCloneGrant { maximum_copy_bytes:0,..original }] {
            let grant=ArtifactStoreOneItemGrant::from_retained(denied);
            assert_eq!(query.advance(grant,4096).unwrap(), LocalInteractionQueryStep::Blocked);
            assert_eq!(query.completed_bytes(), 0);
            assert!(query.page().is_none());
        }
        while query.completed_bytes() < prefix.as_u64().unwrap() {
            let(step,physical)=semio_framework_trace::observe_heap_allocations_on_this_thread(||query.advance(ArtifactStoreOneItemGrant::from_retained(recipient.remaining_grant()),1).unwrap());let progress=step.progress();recipient.record_progress(progress).unwrap();assert_eq!((physical.requested_bytes,physical.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));
            if query.completed_bytes() < prefix.as_u64().unwrap() {
                if let Some(page) = query.page() {
                    let token = page.token.clone();
                    assert!(query.acknowledge(&token));
                }
            }
        }
        query = std::thread::spawn(move || query).join().unwrap();
        query.cancel();
        assert!(query.page().is_none());
        let before = query.completed_bytes();
        assert_eq!(query.advance(ArtifactStoreOneItemGrant::from_retained(recipient.remaining_grant()),4096).unwrap(), LocalInteractionQueryStep::Closing);
        assert_eq!(query.completed_bytes(), before);
        assert_eq!(query.close_step(RetainedCloneGrant { maximum_items:0,..recipient.remaining_grant() }).unwrap().progress(),Default::default());
        close_original_query(&mut store,&mut query,&mut recipient,1_000_000);
    }
}

//#region 🫙️EmptyCapture
/// 🫙️ A capture that completes without a single byte, so its FIRST page is terminal and empty.
/// The fixed page authority must still publish it, accept its exact ACK, retire and reach terminal
/// emptiness — an empty capture is a whole answer, not a missing one.
pub(crate) struct EmptyCapture {
    identity: LocalInteractionIdentity,
    returned: bool,
}

pub(crate) fn empty_capture_for_live_law() -> impl LocalInteractionQueryCapture {
    EmptyCapture { identity: LocalInteractionIdentity { app_instance_id: 5, generation: 2, revision: [4; 32], document_revision: [5; 32], topology_revision: [6; 32] }, returned: false }
}

impl LocalInteractionQueryCapture for EmptyCapture {
    fn identity(&self) -> &LocalInteractionIdentity {
        &self.identity
    }
    fn write_chunk(&mut self, _grant: ArtifactStoreOneItemGrant, _output: &mut [u8]) -> Result<store::ArtifactCanonicalJsonTreeStep, store::ArtifactCanonicalJsonEncodeError> {
        Ok(store::ArtifactCanonicalJsonTreeStep { written_bytes: 0, ownership: RetainedCloneStep::Complete(Default::default()) })
    }
    fn complete(&self) -> bool {
        true
    }
    fn completed_bytes(&self) -> u64 {
        0
    }
    fn cancel(&mut self) {}
    fn begin_close(&mut self) {}
    fn retirement_demands(&self, _body:usize) -> Result<RetirementDemand, ValueError> {
        Ok(RetirementDemand { depth:usize::from(!self.returned), ..Default::default() })
    }
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if self.returned { return Ok(RetainedCloneStep::Complete(Default::default())); }
        if grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        self.returned = true;
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items:1, ..Default::default() }))
    }
    fn terminal_is_empty(&self) -> bool {
        self.returned
    }
}

#[test]
fn local_interaction_query_empty_capture_publishes_its_terminal_page_and_retires() {
    for bytes in [1, 64, 4096] {
        let mut recipient=original_receiving_recipient();let grant=ArtifactStoreOneItemGrant::from_retained(recipient.remaining_grant());
        let mut query = LocalInteractionQuery::new(empty_capture_for_live_law(), 21, 34);
        assert_eq!(query.advance(grant,bytes).unwrap(), LocalInteractionQueryStep::Advanced { emitted_bytes: 0, retired_bytes: 0, ownership:Default::default() });
        let page = query.page().expect("an empty capture still publishes its exact terminal page");
        assert!(page.terminal, "the first page of an empty capture is terminal");
        assert!(page.bytes.is_empty(), "an empty capture carries no bytes");
        let token = page.token.clone();
        assert_eq!(query.advance(grant,bytes).unwrap(), LocalInteractionQueryStep::PageReady);
        assert!(query.acknowledge(&token));
        assert!(!query.acknowledge(&token));
        for _ in 0..64 {
            let(step,physical)=semio_framework_trace::observe_heap_allocations_on_this_thread(||query.close_step(recipient.remaining_grant()).unwrap());let progress=step.progress();recipient.record_progress(progress).unwrap();assert_eq!((physical.requested_bytes,physical.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));if matches!(step,RetainedCloneStep::Complete(_)){break}
        }
        assert!(query.terminal_is_empty(), "an acknowledged empty capture reaches terminal emptiness at bytes={bytes}");
        assert_eq!(query.completed_bytes(), query.retired_bytes());
    }
}
//#endregion 🫙️EmptyCapture

//#region ⚠️PartialEncoderFailure
#[derive(semio_framework_value::RetireOwned)]
struct HostileValue;
#[derive(semio_framework_value::RetireOwned)]
struct HostileRoot {
    first: String,
    second: HostileValue,
}

impl store::ArtifactCanonicalJson for HostileValue {
    fn canonical_json_borrowed_root(&self) -> Result<Option<store::ArtifactCanonicalJsonValue<'_>>, semio_framework_value::ValueError> {
        Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "query.hostile-source"))
    }
}

impl store::ArtifactCanonicalJson for HostileRoot {
    fn canonical_json_borrowed_root(&self) -> Result<Option<store::ArtifactCanonicalJsonValue<'_>>, semio_framework_value::ValueError> {
        use store::{ArtifactCanonicalJsonNode as Node, ArtifactCanonicalJsonObject as Object, ArtifactCanonicalJsonValue as Value};
        Ok(Some(Value::Object(Object::new([("first", Value::Scalar(Node::String(&self.first))), ("second", Value::Source(&self.second))].into_iter()))))
    }
}

struct HostileRetirementFactory;

struct HostileRetirement {
    root: Option<std::sync::Arc<HostileRoot>>,
    bytes: Vec<u8>,
}

impl store::SnapshotRetirementFactory<HostileRoot> for HostileRetirementFactory {
    fn retirement_birth_bytes(&self, _snapshot: &std::sync::Arc<HostileRoot>) -> usize { std::mem::size_of::<HostileRetirement>() }

    fn retire(&self, root: std::sync::Arc<HostileRoot>, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress), (semio_framework_value::ValueError, std::sync::Arc<HostileRoot>)> {
        match (RetainedCloneBirthDemand { capacity_bytes: std::mem::size_of::<HostileRetirement>(), depth: 1 }).admit(grant) {
            Ok(progress) => Ok((Box::new(HostileRetirement { root: Some(root), bytes: Vec::new() }), progress)),
            Err(error) => Err((error, root)),
        }
    }
}

impl HostileRetirement {
    fn demands(&self) -> RetirementDemand {
        if self.terminal_is_empty() {
            Default::default()
        } else if self.root.is_some() {
            RetirementDemand { depth: 1, ..Default::default() }
        } else if !self.bytes.is_empty() {
            RetirementDemand { copy_bytes: 1, depth: 1, ..Default::default() }
        } else {
            RetirementDemand { release_bytes: self.bytes.capacity(), depth: 1, ..Default::default() }
        }
    }
}

impl ErasedSnapshotRetirement for HostileRetirement {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, semio_framework_value::ValueError> {
        let idle = RetainedCloneProgress::default();
        if self.terminal_is_empty() {
            return Ok(RetainedCloneStep::Complete(idle));
        }
        let demand = self.demands();
        if grant.maximum_depth < demand.depth {
            return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "hostile retirement exceeds admitted depth"));
        }
        if grant.maximum_items == 0 || grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_release_bytes < demand.release_bytes {
            return Ok(RetainedCloneStep::Progress(idle));
        }
        let progress = if let Some(root) = self.root.take() {
            if let Some(root) = std::sync::Arc::into_inner(root) {
                self.bytes = root.first.into_bytes();
            }
            RetainedCloneProgress { copied_items: 1, ..idle }
        } else if !self.bytes.is_empty() {
            let copied_bytes = grant.maximum_copy_bytes.min(self.bytes.len());
            self.bytes.truncate(self.bytes.len() - copied_bytes);
            RetainedCloneProgress { copied_items: 1, copied_bytes, ..idle }
        } else {
            let released_bytes = self.bytes.capacity();
            self.bytes = Vec::new();
            RetainedCloneProgress { copied_items: 1, released_bytes, ..idle }
        };
        Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(progress) } else { RetainedCloneStep::Progress(progress) })
    }
    fn terminal_is_empty(&self) -> bool {
        self.root.is_none() && self.bytes.is_empty() && self.bytes.capacity() == 0
    }
    fn next_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.demands().copy_bytes)
    }
    fn next_capacity_byte_demand(&self, _maximum_body_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.demands().capacity_bytes)
    }
    fn next_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.demands().release_bytes)
    }
    fn next_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.demands().depth)
    }
}

struct HostileCapture {
    reader: store::ArtifactCanonicalJsonReader<HostileRoot>,
    identity: LocalInteractionIdentity,
}

pub(crate) fn hostile_capture_for_live_law() -> impl LocalInteractionQueryCapture {
    HostileCapture {
        reader: store::ArtifactCanonicalJsonReader::new(std::sync::Arc::new(HostileRoot { first: "retained✓".into(), second: HostileValue }), std::sync::Arc::new(semio_framework_value::retirement::SharedValueRetirementFactory::<HostileRoot>::default())),
        identity: LocalInteractionIdentity { app_instance_id: 1, generation: 1, revision: [1; 32], document_revision: [2; 32], topology_revision: [3; 32] },
    }
}

impl LocalInteractionQueryCapture for HostileCapture {
    fn identity(&self) -> &LocalInteractionIdentity {
        &self.identity
    }
    fn write_chunk(&mut self, grant: ArtifactStoreOneItemGrant, output: &mut [u8]) -> Result<store::ArtifactCanonicalJsonTreeStep, store::ArtifactCanonicalJsonEncodeError> {
        self.reader.encode_chunk(grant, output)
    }
    fn complete(&self) -> bool {
        self.reader.is_complete()
    }
    fn completed_bytes(&self) -> u64 {
        self.reader.completed_bytes()
    }
    fn cancel(&mut self) {
        self.reader.cancel();
        self.reader.begin_close();
    }
    fn begin_close(&mut self) {
        self.reader.begin_close();
    }
    fn retirement_demands(&self, body:usize)->Result<RetirementDemand,ValueError>{self.reader.retirement_demands(body)}
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        self.reader.close_step(grant)
    }
    fn terminal_is_empty(&self) -> bool {
        self.reader.terminal_is_empty()
    }
}

#[test]
fn local_interaction_query_partial_encoder_failure_keeps_exact_byte_ownership() {
    let laws: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../🔨️modules/📡️replication/📡️wire/🏠️local-interaction/🧫️fixtures/📃️query/🔣️.json")).unwrap();
    for bytes in [1, 64, 4096] {
        let root = std::sync::Arc::new(HostileRoot { first: laws["partialError"]["first"].as_str().unwrap().into(), second: HostileValue });
        let identity = LocalInteractionIdentity { app_instance_id: 1, generation: 1, revision: [1; 32], document_revision: [2; 32], topology_revision: [3; 32] };
        let source = HostileCapture { reader: store::ArtifactCanonicalJsonReader::new(root, std::sync::Arc::new(semio_framework_value::retirement::SharedValueRetirementFactory::<HostileRoot>::default())), identity };
        let mut query = LocalInteractionQuery::new(source, 9, 42);
        let mut recipient=original_receiving_recipient();
        let mut failed = false;
        for _ in 0..10_000 {
            let(result,physical)=semio_framework_trace::observe_heap_allocations_on_this_thread(||query.advance(ArtifactStoreOneItemGrant::from_retained(recipient.remaining_grant()),bytes));let progress=match &result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};recipient.record_progress(progress).unwrap();assert_eq!((physical.requested_bytes,physical.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));
            if let Err(error) = result {
                assert_eq!(error.to_string(), laws["partialError"]["error"].as_str().unwrap());
                failed = true;
                break;
            }
            if let Some(page) = query.page() {
                let token = page.token.clone();
                assert!(query.acknowledge(&token));
            }
        }
        assert!(failed);
        assert_eq!(query.completed_bytes(), laws["partialError"]["expectedPrefix"].as_str().unwrap().len() as u64);
        assert!(query.page().is_none());
        assert_eq!(query.advance(ArtifactStoreOneItemGrant::from_retained(recipient.remaining_grant()),bytes).unwrap(), LocalInteractionQueryStep::Closing);
        for _ in 0..10_000 {
            let(step,physical)=semio_framework_trace::observe_heap_allocations_on_this_thread(||query.close_step(recipient.remaining_grant()).unwrap());let progress=step.progress();recipient.record_progress(progress).unwrap();assert_eq!((physical.requested_bytes,physical.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));if matches!(step,RetainedCloneStep::Complete(_)){break}
        }
        assert!(query.terminal_is_empty());
        assert_eq!(query.completed_bytes(), query.retired_bytes());
    }
}
//#endregion ⚠️PartialEncoderFailure
