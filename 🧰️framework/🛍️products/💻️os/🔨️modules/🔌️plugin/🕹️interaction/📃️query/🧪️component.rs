//! 🧪️ Actual fixed-page query output, exact ACK authority, cancellation, and Store read return.

use super::*;
use protocol::InteractionState;
use store::{ArtifactStore, ErasedSnapshotRetirement, SpaceMember};
use crate::app::InteractionConfigMutation;
use crate::local_interaction::retirement::interaction_store_owners;

type InteractionStore = ArtifactStore<InteractionState, InteractionConfigMutation>;

async fn fixture(source_case: &str) -> (InteractionStore, LocalInteractionQuery, Vec<u8>) {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../🔨️modules/📡️replication/📡️wire/🏠️local-interaction/🧪️fixtures/🔣️local-interaction.json")).unwrap();
    let row = fixture["cases"].as_array().unwrap().iter().find(|row| row["id"] == source_case).unwrap();
    let mut state = row["expected"].clone();
    state["hover"] = serde_json::json!({"private": {"channel": "pointer", "ids": ["not-captured"]}});
    let state: InteractionState = serde_json::from_value(state).unwrap();
    let envelope = store::create_document_envelope::<InteractionState, InteractionConfigMutation>("framework.interaction", "local-query-test", state, None);
    let mut store = InteractionStore::new(envelope).await.unwrap();
    store.install_member_store_owners_exact(interaction_store_owners());
    let identity = LocalInteractionIdentity { app_instance_id: 7, generation: store.generation_now(), revision: store.content_revision_now(), document_revision: [2; 32], topology_revision: [3; 32] };
    let expected = serde_json::to_vec(&serde_json::json!({"identity": identity, "state": row["expected"]})).unwrap();
    let capture = LocalInteractionCaptureCursor::new(store.snapshot_read().unwrap(), identity);
    (store, LocalInteractionQuery::new(capture, 13, 41), expected)
}

fn close(store: &mut InteractionStore, query: &mut LocalInteractionQuery, bytes: usize) {
    let mut retirement: Option<Box<dyn ErasedSnapshotRetirement>> = None;
    for _ in 0..1_000_000 {
        match query.close_step(ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: bytes }).unwrap() {
            SnapshotRetirementStep::Complete => assert!(query.terminal_is_empty()),
            SnapshotRetirementStep::Pending { released_items, released_bytes } => assert!(released_items <= 1 && released_bytes <= bytes),
            SnapshotRetirementStep::Blocked => {},
        }
        if retirement.is_none() { retirement = store.take_returned_snapshot_read_retirement().unwrap(); }
        if let Some(active) = retirement.as_mut() {
            match active.close_step(1, bytes).unwrap() {
                SnapshotRetirementStep::Complete => { assert!(active.terminal_is_empty()); retirement = None; },
                SnapshotRetirementStep::Pending { released_items, released_bytes } => assert!(released_items <= 1 && released_bytes <= bytes),
                SnapshotRetirementStep::Blocked => {},
            }
        }
        if query.terminal_is_empty() && retirement.is_none() && store.snapshot_read_leases_terminal_is_empty() { break; }
    }
    assert!(query.terminal_is_empty());
    assert!(retirement.is_none());
    assert!(store.snapshot_read_leases_terminal_is_empty());
    assert_eq!(query.completed_bytes(), query.retired_bytes());
    for _ in 0..1_000_000 {
        if matches!(store.close_owned_step(1, bytes).unwrap(), SnapshotRetirementStep::Complete) {
            assert!(store.close_owned_terminal_is_empty()); return;
        }
    }
    panic!("query Store did not close");
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
    let laws: serde_json::Value = serde_json::from_str(include_str!("../../../../../../🔨️modules/📡️replication/📡️wire/🏠️local-interaction/🧪️fixtures/📃️query.json")).unwrap();
    for source in laws["sourceCases"].as_array().unwrap() {
        for bytes in [1, 64, 4096] {
            let (mut store, mut query, expected) = fixture(source.as_str().unwrap()).await;
            let grant = ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: bytes };
            let mut actual = Vec::new();
            let mut ordinal = 0;
            for _ in 0..500_000 {
                query.advance(grant).unwrap();
                let Some(page) = query.page() else { continue };
                assert_eq!(page.token.ordinal, ordinal);
                assert!(page.bytes.len() <= bytes.min(LOCAL_INTERACTION_QUERY_PAGE_BYTES));
                actual.extend_from_slice(page.bytes);
                let terminal = page.terminal;
                let token = page.token.clone();
                let before = query.completed_bytes();
                assert_eq!(query.advance(grant).unwrap(), LocalInteractionQueryStep::PageReady);
                assert_eq!(query.completed_bytes(), before);
                for field in laws["wrongAcknowledgements"].as_array().unwrap() { assert!(!query.acknowledge(&wrong_token(token.clone(), field.as_str().unwrap()))); }
                assert!(query.acknowledge(&token));
                assert!(!query.acknowledge(&token));
                assert!(query.page().is_none());
                ordinal += 1;
                if terminal { break; }
            }
            assert_eq!(actual, expected);
            assert!(!query.terminal_is_empty());
            close(&mut store, &mut query, bytes);
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn local_interaction_query_zero_grants_cancel_and_worker_transfer() {
    let laws: serde_json::Value = serde_json::from_str(include_str!("../../../../../../🔨️modules/📡️replication/📡️wire/🏠️local-interaction/🧪️fixtures/📃️query.json")).unwrap();
    for prefix in laws["cancelAfterBytes"].as_array().unwrap() {
        let (mut store, mut query, _) = fixture("semantic-unicode-over-page").await;
        for grant in [ArtifactStoreOneItemGrant { maximum_items: 0, maximum_bytes: 4096 }, ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 0 }] {
            assert_eq!(query.advance(grant).unwrap(), LocalInteractionQueryStep::Blocked);
            assert_eq!(query.completed_bytes(), 0);
            assert!(query.page().is_none());
        }
        while query.completed_bytes() < prefix.as_u64().unwrap() {
            query.advance(ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 1 }).unwrap();
            if query.completed_bytes() < prefix.as_u64().unwrap() {
                if let Some(page) = query.page() { let token = page.token.clone(); assert!(query.acknowledge(&token)); }
            }
        }
        query = std::thread::spawn(move || query).join().unwrap();
        query.cancel();
        assert!(query.page().is_none());
        let before = query.completed_bytes();
        assert_eq!(query.advance(ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 4096 }).unwrap(), LocalInteractionQueryStep::Closing);
        assert_eq!(query.completed_bytes(), before);
        assert!(matches!(query.close_step(ArtifactStoreOneItemGrant { maximum_items: 0, maximum_bytes: 4096 }).unwrap(), SnapshotRetirementStep::Blocked));
        close(&mut store, &mut query, 1);
    }
}

//#region ⚠️PartialEncoderFailure
struct HostileValue;
struct HostileRoot { first: String, second: HostileValue }
struct HostileRetirementFactory;
struct HostileRetirement { root: Option<std::sync::Arc<HostileRoot>>, bytes: Vec<u8> }

impl store::ArtifactCanonicalJson for HostileValue {
    fn canonical_json_borrowed_root(&self) -> Result<Option<store::ArtifactCanonicalJsonValue<'_>>, String> { Err("query.hostile-source".into()) }
}

impl store::ArtifactCanonicalJson for HostileRoot {
    fn canonical_json_borrowed_root(&self) -> Result<Option<store::ArtifactCanonicalJsonValue<'_>>, String> {
        use store::{ArtifactCanonicalJsonNode as Node, ArtifactCanonicalJsonObject as Object, ArtifactCanonicalJsonValue as Value};
        Ok(Some(Value::Object(Object::new([("first", Value::Scalar(Node::String(&self.first))), ("second", Value::Source(&self.second))].into_iter()))))
    }
}

impl store::SnapshotRetirementFactory<HostileRoot> for HostileRetirementFactory {
    fn retire(&self, root: std::sync::Arc<HostileRoot>) -> Box<dyn ErasedSnapshotRetirement> { Box::new(HostileRetirement { root: Some(root), bytes: Vec::new() }) }
}

impl ErasedSnapshotRetirement for HostileRetirement {
    fn close_step(&mut self, items: usize, bytes: usize) -> Result<SnapshotRetirementStep, String> {
        if self.terminal_is_empty() { return Ok(SnapshotRetirementStep::Complete); }
        if items == 0 { return Ok(SnapshotRetirementStep::Blocked); }
        if let Some(root) = self.root.take() {
            if let Some(root) = std::sync::Arc::into_inner(root) { self.bytes = root.first.into_bytes(); }
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if !self.bytes.is_empty() {
            if bytes == 0 { return Ok(SnapshotRetirementStep::Blocked); }
            let released_bytes = bytes.min(self.bytes.len());
            self.bytes.truncate(self.bytes.len() - released_bytes);
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes });
        }
        self.bytes = Vec::new();
        Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
    }
    fn terminal_is_empty(&self) -> bool { self.root.is_none() && self.bytes.is_empty() && self.bytes.capacity() == 0 }
}

struct HostileCapture { reader: store::ArtifactCanonicalJsonReader<HostileRoot>, identity: LocalInteractionIdentity }

pub(crate) fn hostile_capture_for_live_law() -> impl LocalInteractionQueryCapture {
    HostileCapture {
        reader: store::ArtifactCanonicalJsonReader::new(std::sync::Arc::new(HostileRoot { first: "retained✓".into(), second: HostileValue }), std::sync::Arc::new(HostileRetirementFactory)),
        identity: LocalInteractionIdentity { app_instance_id: 1, generation: 1, revision: [1; 32], document_revision: [2; 32], topology_revision: [3; 32] },
    }
}

impl LocalInteractionQueryCapture for HostileCapture {
    fn identity(&self) -> &LocalInteractionIdentity { &self.identity }
    fn write_chunk(&mut self, grant: ArtifactStoreOneItemGrant, output: &mut [u8]) -> Result<usize, store::ArtifactCanonicalJsonEncodeError> { self.reader.encode_chunk(grant, output) }
    fn complete(&self) -> bool { self.reader.is_complete() }
    fn completed_bytes(&self) -> u64 { self.reader.completed_bytes() }
    fn cancel(&mut self) { self.reader.cancel(); self.reader.begin_close(); }
    fn begin_close(&mut self) { self.reader.begin_close(); }
    fn close_step(&mut self, grant: ArtifactStoreOneItemGrant) -> Result<SnapshotRetirementStep, String> { self.reader.close_step(grant) }
    fn terminal_is_empty(&self) -> bool { self.reader.terminal_is_empty() }
}

#[test]
fn local_interaction_query_partial_encoder_failure_keeps_exact_byte_ownership() {
    let laws: serde_json::Value = serde_json::from_str(include_str!("../../../../../../🔨️modules/📡️replication/📡️wire/🏠️local-interaction/🧪️fixtures/📃️query.json")).unwrap();
    for bytes in [1, 64, 4096] {
        let root = std::sync::Arc::new(HostileRoot { first: laws["partialError"]["first"].as_str().unwrap().into(), second: HostileValue });
        let identity = LocalInteractionIdentity { app_instance_id: 1, generation: 1, revision: [1; 32], document_revision: [2; 32], topology_revision: [3; 32] };
        let source = HostileCapture { reader: store::ArtifactCanonicalJsonReader::new(root, std::sync::Arc::new(HostileRetirementFactory)), identity };
        let mut query = LocalInteractionQuery::new(source, 9, 42);
        let grant = ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: bytes };
        let mut failed = false;
        for _ in 0..10_000 {
            if let Err(error) = query.advance(grant) { assert_eq!(error, laws["partialError"]["error"].as_str().unwrap()); failed = true; break; }
            if let Some(page) = query.page() { let token = page.token.clone(); assert!(query.acknowledge(&token)); }
        }
        assert!(failed);
        assert_eq!(query.completed_bytes(), laws["partialError"]["expectedPrefix"].as_str().unwrap().len() as u64);
        assert!(query.page().is_none());
        assert_eq!(query.advance(grant).unwrap(), LocalInteractionQueryStep::Closing);
        for _ in 0..10_000 {
            if matches!(query.close_step(ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 1 }).unwrap(), SnapshotRetirementStep::Complete) { break; }
        }
        assert!(query.terminal_is_empty());
        assert_eq!(query.completed_bytes(), query.retired_bytes());
    }
}
//#endregion ⚠️PartialEncoderFailure
