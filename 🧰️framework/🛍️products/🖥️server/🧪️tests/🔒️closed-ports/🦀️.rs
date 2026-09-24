//! 🔒️ Every port of this product, closed over a two-variant enum **from another crate**.
//!
//! This is an integration test, so it links `server` exactly as hub will: the eight ports that
//! declare `impl Future<..> + Send` are reached through `use server::__semio_dispatch_<Trait>;`,
//! the cross-crate half of the `dyn_enum_close!` recipe that a same-crate call site may not use
//! (rust-lang/rust#52234). It is the compile-time gate for step 4 of the Wave-3 memo: if closing a
//! set of hub deciders, hub resolvers or hub storage backends could not be generated, it would fail
//! here rather than in the middle of the hub crate.
//!
//! `Rung<N>` is one implementation per port written once and instantiated at two different const
//! parameters, so every closed set below has two genuinely distinct variant types — the case a
//! single `match` could not express while these ports were opaque-future-returning — without eight
//! pairs of hand-copied fixtures. Each port answers its own `N` at runtime, so a test can prove
//! which arm ran rather than only that the code compiled.

use semio_framework_dispatch_macros::dyn_enum_close;

use protocol::codec::ids::ContentHash;
use server::authority::{ActorState, Decider, Decision, DecisionContext};
use server::contract::{ActorKey, CommandEnvelope, CommandReceipt, EventRecord, IdempotencyKey, Principal, QueryConsistency, QueryEnvelope, QueryId, QueryResult, Rejection, Revision, Scope, SessionId, TenantId};
use server::gateway::{document_socket_identity, DocumentAuthority, DocumentFrames, DocumentHandshake, DocumentSocketIdentity, QueryHandler, ServerError};
use server::policy::{Credential, PrincipalResolver, Resolved};
use server::storage::{AuthorityStore, BlobStore, Lease, OutboxEntry, ProjectionStore, SessionRecord, SessionStore, StorageError};
use server::{
    __semio_dispatch_AuthorityStore, __semio_dispatch_BlobStore, __semio_dispatch_Decider, __semio_dispatch_DocumentAuthority, __semio_dispatch_PrincipalResolver, __semio_dispatch_ProjectionStore,
    __semio_dispatch_QueryHandler, __semio_dispatch_SessionStore,
};

//#region 🔖️One implementation, two instantiations

/// 🪜️ A stand-in implementation of every port, distinguished only by `N`, which each method reports
/// so a delegation can be traced to the arm that served it.
pub struct Rung<const N: u8>;

/// 🏷️ The name `Rung<N>` answers with, without allocating a `String` a `-> &str` port could not
/// return.
const fn rung_name<const N: u8>() -> &'static str {
    if N == 1 {
        "rung-1"
    } else {
        "rung-2"
    }
}

impl<const N: u8> AuthorityStore for Rung<N> {
    async fn receipt(&self, _key: &IdempotencyKey) -> Result<Option<CommandReceipt>, StorageError> {
        Ok(None)
    }

    async fn record_receipt(&mut self, _key: &IdempotencyKey, _receipt: &CommandReceipt) -> Result<(), StorageError> {
        Ok(())
    }

    async fn append_events(&mut self, _actor: &ActorKey, events: &[EventRecord], _outbox: &[OutboxEntry]) -> Result<u64, StorageError> {
        Ok(u64::from(N) + events.len() as u64)
    }

    async fn events_since(&self, _actor: &ActorKey, _since: u64) -> Result<Vec<EventRecord>, StorageError> {
        Ok(Vec::new())
    }

    async fn last_seq(&self, _actor: &ActorKey) -> Result<u64, StorageError> {
        Ok(u64::from(N))
    }

    async fn put_snapshot(&mut self, _actor: &ActorKey, _revision: Revision, _bytes: Vec<u8>) -> Result<(), StorageError> {
        Ok(())
    }

    async fn snapshot(&self, _actor: &ActorKey) -> Result<Option<(Revision, Vec<u8>)>, StorageError> {
        Ok(None)
    }

    async fn enqueue_outbox(&mut self, _entries: Vec<OutboxEntry>) -> Result<(), StorageError> {
        Ok(())
    }

    async fn pending_outbox(&self, _limit: usize) -> Result<Vec<OutboxEntry>, StorageError> {
        Ok(Vec::new())
    }

    async fn mark_outbox_delivered(&mut self, _ids: &[u64]) -> Result<(), StorageError> {
        Ok(())
    }

    async fn acquire_lease(&mut self, _actor: &ActorKey, holder: &str) -> Result<Lease, StorageError> {
        Ok(Lease { epoch: u64::from(N), holder: holder.to_owned() })
    }

    async fn validate_lease(&self, _actor: &ActorKey, lease: &Lease) -> bool {
        lease.epoch == u64::from(N)
    }
}

impl<const N: u8> ProjectionStore for Rung<N> {
    async fn put(&mut self, _projection: &str, _key: &str, _value: Vec<u8>) -> Result<(), StorageError> {
        Ok(())
    }

    async fn get(&self, _projection: &str, _key: &str) -> Option<Vec<u8>> {
        Some(vec![N])
    }

    async fn list(&self, _projection: &str, _prefix: &str) -> Vec<(String, Vec<u8>)> {
        Vec::new()
    }

    async fn checkpoint(&self, _projection: &str) -> u64 {
        u64::from(N)
    }

    async fn set_checkpoint(&mut self, _projection: &str, _seq: u64) -> Result<(), StorageError> {
        Ok(())
    }

    async fn clear(&mut self, _projection: &str) -> Result<(), StorageError> {
        Ok(())
    }
}

impl<const N: u8> BlobStore for Rung<N> {
    async fn put(&mut self, _hash: ContentHash, _bytes: &[u8]) -> Result<(), StorageError> {
        Ok(())
    }

    async fn get(&self, _hash: &ContentHash) -> Option<Vec<u8>> {
        Some(vec![N])
    }

    async fn has(&self, _hash: &ContentHash) -> bool {
        N == 1
    }
}

impl<const N: u8> SessionStore for Rung<N> {
    async fn create(&mut self, _session: SessionRecord) -> Result<(), StorageError> {
        Ok(())
    }

    async fn get(&self, _id: &SessionId) -> Option<SessionRecord> {
        None
    }

    async fn delete(&mut self, _id: &SessionId) -> Result<(), StorageError> {
        Ok(())
    }

    async fn revoke_principal(&mut self, _principal: &Principal) -> Result<usize, StorageError> {
        Ok(usize::from(N))
    }
}

impl<const N: u8> PrincipalResolver for Rung<N> {
    async fn resolve(&self, credential: &Credential) -> Option<Resolved> {
        credential.bearer.as_deref().filter(|bearer| *bearer == rung_name::<N>()).map(|bearer| Resolved {
            principal: Principal::User { id: bearer.to_owned() },
            session: None,
            device: None,
            via: rung_name::<N>().to_owned(),
            actor: Some(bearer.to_owned()),
        })
    }

    async fn name(&self) -> &str {
        rung_name::<N>()
    }
}

impl<const N: u8> Decider for Rung<N> {
    async fn actor_kind(&self) -> &str {
        rung_name::<N>()
    }

    async fn decide(&self, _state: &ActorState, command: &CommandEnvelope, _context: &DecisionContext) -> Decision {
        Decision::Reject(Rejection::UnknownCommandKind { command_kind: command.kind.clone() })
    }

    async fn evolve(&self, state: &mut ActorState, _event: &EventRecord) {
        state.bytes = vec![N];
    }
}

impl<const N: u8> DocumentAuthority for Rung<N> {
    async fn handshake(&self) -> DocumentHandshake {
        DocumentHandshake::ServerFirst
    }

    async fn bind_socket(&self, _scope: &Scope, resolved: &Resolved) -> Result<DocumentSocketIdentity, ServerError> {
        document_socket_identity(resolved)
    }

    async fn welcome(&self, _scope: &Scope, _actor: &str, _resume: Option<&str>, _hello: Option<&[u8]>) -> Result<Vec<Vec<u8>>, ServerError> {
        Ok(vec![vec![N]])
    }

    async fn submit_frame(&self, _scope: &Scope, _actor: &str, _principal: &Principal, frame: &[u8]) -> Result<DocumentFrames, ServerError> {
        Ok(DocumentFrames::mirrored(vec![frame.to_vec()]))
    }
}

impl<const N: u8> QueryHandler for Rung<N> {
    async fn kind(&self) -> &str {
        rung_name::<N>()
    }

    async fn handle<P: ProjectionStore>(&self, _envelope: &QueryEnvelope, projections: &P) -> Result<QueryResult, ServerError> {
        Ok(QueryResult::Snapshot { value: projections.get("any", "any").await.unwrap_or_default(), frontier: None })
    }
}

//#endregion

//#region 🔖️The eight closed sets

dyn_enum_close! {
    /// 🏛️ Two authoritative histories behind one type — the shape a deployment that shards or
    /// migrates its authority backend needs.
    pub enum ClosedAuthorityStores: AuthorityStore {
        First(Rung<1>),
        Second(Rung<2>),
    }
}

dyn_enum_close! {
    /// 🔭️ Two read-model backends behind one type.
    pub enum ClosedProjectionStores: ProjectionStore {
        First(Rung<1>),
        Second(Rung<2>),
    }
}

dyn_enum_close! {
    /// 🧱️ Two blob backends behind one type.
    pub enum ClosedBlobStores: BlobStore {
        First(Rung<1>),
        Second(Rung<2>),
    }
}

dyn_enum_close! {
    /// 🎫️ Two session backends behind one type.
    pub enum ClosedSessionStores: SessionStore {
        First(Rung<1>),
        Second(Rung<2>),
    }
}

dyn_enum_close! {
    /// 🪜️ Two authentication rungs behind one type — the set an instance most obviously has
    /// several of.
    pub enum ClosedResolvers: PrincipalResolver {
        First(Rung<1>),
        Second(Rung<2>),
    }
}

dyn_enum_close! {
    /// ⚖️ Two deterministic cores behind one type, one per actor kind.
    pub enum ClosedDeciders: Decider {
        First(Rung<1>),
        Second(Rung<2>),
    }
}

dyn_enum_close! {
    /// 📄️ Two replication engines behind one type.
    pub enum ClosedDocuments: DocumentAuthority {
        First(Rung<1>),
        Second(Rung<2>),
    }
}

dyn_enum_close! {
    /// ❓️ Two read handlers behind one type, one per query kind.
    pub enum ClosedQueries: QueryHandler {
        First(Rung<1>),
        Second(Rung<2>),
    }
}

//#endregion

fn actor() -> ActorKey {
    ActorKey { tenant: TenantId("t".into()), kind: "counter".to_string(), id: "a".to_string() }
}

#[semio_framework_async_macros::async_test]
async fn each_storage_port_dispatches_to_the_arm_it_was_given() {
    assert_eq!(ClosedAuthorityStores::First(Rung::<1>).last_seq(&actor()).await, Ok(1));
    assert_eq!(ClosedAuthorityStores::Second(Rung::<2>).last_seq(&actor()).await, Ok(2));
    assert_eq!(ClosedProjectionStores::First(Rung::<1>).checkpoint("p").await, 1);
    assert_eq!(ClosedProjectionStores::Second(Rung::<2>).checkpoint("p").await, 2);
    assert!(ClosedBlobStores::First(Rung::<1>).has(&ContentHash([0; 32])).await);
    assert!(!ClosedBlobStores::Second(Rung::<2>).has(&ContentHash([0; 32])).await);
    assert_eq!(ClosedSessionStores::First(Rung::<1>).revoke_principal(&Principal::Anonymous).await, Ok(1));
    assert_eq!(ClosedSessionStores::Second(Rung::<2>).revoke_principal(&Principal::Anonymous).await, Ok(2));
}

#[semio_framework_async_macros::async_test]
async fn each_remaining_port_dispatches_to_the_arm_it_was_given() {
    assert_eq!(ClosedResolvers::First(Rung::<1>).name().await, "rung-1");
    assert_eq!(ClosedResolvers::Second(Rung::<2>).name().await, "rung-2");
    assert_eq!(ClosedDeciders::First(Rung::<1>).actor_kind().await, "rung-1");
    assert_eq!(ClosedDeciders::Second(Rung::<2>).actor_kind().await, "rung-2");
    assert_eq!(ClosedDocuments::First(Rung::<1>).welcome(&Scope("s".into()), "a", None, None).await.expect("the first arm answers"), vec![vec![1]]);
    assert_eq!(ClosedDocuments::Second(Rung::<2>).welcome(&Scope("s".into()), "a", None, None).await.expect("the second arm answers"), vec![vec![2]]);
    assert_eq!(ClosedQueries::First(Rung::<1>).kind().await, "rung-1");
    assert_eq!(ClosedQueries::Second(Rung::<2>).kind().await, "rung-2");
}

#[semio_framework_async_macros::async_test]
async fn a_mutating_port_mutates_through_the_delegate() {
    let mut stores = ClosedAuthorityStores::Second(Rung::<2>);
    assert_eq!(stores.acquire_lease(&actor(), "holder").await.map(|lease| lease.epoch), Ok(2));
    let deciders = ClosedDeciders::First(Rung::<1>);
    let mut state = ActorState::default();
    deciders.evolve(&mut state, &EventRecord { stream: actor(), seq: 1, hlc: Default::default(), kind: "k".into(), payload: Vec::new() }).await;
    assert_eq!(state.bytes, vec![1]);
}

/// 🧵️ The property the whole port shape exists for, asserted where hub will rely on it: a future
/// obtained through a generic `S: AuthorityStore` bound must be `Send`, so an axum handler or a
/// spawned socket task can hold it across an await.
#[semio_framework_async_macros::async_test]
async fn a_delegated_future_stays_send_behind_a_generic_bound() {
    async fn head<S: AuthorityStore>(store: &S) -> u64 {
        store.last_seq(&actor()).await.unwrap_or_default()
    }
    fn assert_send<T: Send>(_value: &T) {}

    let stores = ClosedAuthorityStores::Second(Rung::<2>);
    let pending = head(&stores);
    assert_send(&pending);
    assert_eq!(pending.await, 2);
}

/// 🧩️ A generic METHOD on a closed port — `QueryHandler::handle<P>` is the only one in the product,
/// and the one shape a delegate must reproduce with its own type parameter intact.
#[semio_framework_async_macros::async_test]
async fn a_generic_method_delegates_with_its_type_parameter_intact() {
    let queries = ClosedQueries::First(Rung::<1>);
    let projections = ClosedProjectionStores::Second(Rung::<2>);
    let envelope = QueryEnvelope {
        query_id: QueryId("q".into()),
        kind: "rung-1".into(),
        version: 1,
        scope: Scope("s".into()),
        principal: Principal::Anonymous,
        arguments: Vec::new(),
        consistency: QueryConsistency::Local,
        cursor: None,
    };
    let answer = queries.handle(&envelope, &projections).await.expect("the delegate answers");
    assert_eq!(answer, QueryResult::Snapshot { value: vec![2], frontier: None });
}
