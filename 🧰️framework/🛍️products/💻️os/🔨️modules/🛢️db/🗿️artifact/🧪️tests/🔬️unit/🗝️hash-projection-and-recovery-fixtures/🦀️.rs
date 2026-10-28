/// 🗝️ Hash projection and recovery fixtures: the storage, document and stored json helpers, the hash projection, diff and mutation, the retirement factories and the committed recovery store and journal helpers.

async fn storage() -> StdArc<db_storage::DbBackend> {
    StdArc::new(db_storage::DbBackend::Memory(db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap()))
}

async fn document_id() -> protocol::ArtifactId {
    protocol::ArtifactId("doc-1".to_string())
}

async fn stored_json(mut bytes: db_query::QueryBytes) -> serde_json::Value {
    let mut raw = Vec::with_capacity(bytes.len());
    for fragment in bytes.fragments() {
        raw.extend_from_slice(fragment);
    }
    while bytes.close_step().unwrap().is_some() {}
    decode_pathmap_json(&raw).await.expect("stored json value")
}

async fn envelope(id: &str, deps: &[&str], actor: &str, entries: &[(&str, serde_json::Value)]) -> protocol::MutationEnvelope {
    let object: Vec<(String, DslValue)> = entries.iter().map(|(path, value)| (path.to_string(), DslValue::from(value))).collect();
    protocol::MutationEnvelope {
        mutation_id: protocol::MutationId(id.to_string()),
        document_id: document_id().await,
        actor: protocol::ActorId(actor.to_string()),
        dependencies: deps.iter().map(|dep| protocol::MutationId((*dep).to_string())).collect(),
        observed: None,
        target: Vec::new(),
        diff: protocol::ArtifactDiff { schema: protocol::SchemaId(DB_PATHMAP_SCHEMA.to_string()), payload: encode_pathmap(&DslValue::Object(object)).await },
        inverse: protocol::InverseMutation { schema: protocol::SchemaId(DB_PATHMAP_SCHEMA.to_string()), payload: encode_pathmap(&DslValue::Object(vec![])).await },
        timestamp: protocol::HybridLogicalTimestamp::new(0, 0),
        transaction: None, verb: None, line: None,
    }
}
//#region 🔖️Actor
async fn journal_authority() -> (ArtifactAuthority, StdArc<db_storage::DbBackend>, StdArc<semio_framework_async::WorkerPool>) {
    let pool = Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::InteractiveNative, 3)));
    let storage = storage().await;
    let engine_storage = storage.clone();
    let authority = ArtifactAuthority::spawn(pool.clone(), move || async move { ArtifactEngine::create_retained(protocol::ArtifactId("map-a".to_string()), engine_storage, ArtifactEngineConfig::default(), 0).await.map(Box::new) }, MailboxCapacities::uniform(16)).await.unwrap();
    (authority, storage, pool)
}

fn journal_grant() -> store::os_store::one_item::ArtifactStoreOneItemGrant {
    store::os_store::one_item::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_copy_bytes: 4096, maximum_capacity_bytes: store::durable_group::DURABLE_OWNED_GROUP_STRUCTURAL_MAX_BYTES, maximum_release_bytes: store::durable_group::DURABLE_OWNED_GROUP_EVENT_MAX_BYTES, maximum_depth: 64 }
}
//#region 🔖️RecoveryFixtureStore
/// #⃣ The smallest concrete projection/operation pair a durable-group recovery law can open real
/// `store::ArtifactStore`s over: a projection that IS one content hash and an operation that
/// overwrites it (its `inverse` restores the prior hash). Test-only fixture.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
pub struct HashProjection {
    pub latest_hash: [u8; 32],
}

impl semio_framework_schema_composition::ArtifactCompositionFields for HashProjection {
    fn visit_child_refs<'a, V: semio_framework_schema_composition::ChildRefVisitor<'a>>(&'a self, _visitor: &mut V) -> Result<(), V::Error> {
        Ok(())
    }
}

impl pack::snapshot::ArtifactDsl for HashProjection {
    const EXTENSION: &'static str = "dbhash";

    fn parse_dsl(text: &str) -> Result<HashProjection, semio_framework_diagnostic::TextError> {
        let trimmed = text.trim();
        if trimmed.len() != 64 || !trimmed.bytes().all(|byte| byte.is_ascii_hexdigit()) {
            return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected 64 lowercase hex characters", semio_framework_diagnostic::TextSpan::at(1, 1)));
        }
        let mut latest_hash = [0u8; 32];
        for (index, slot) in latest_hash.iter_mut().enumerate() {
            *slot = u8::from_str_radix(&trimmed[index * 2..index * 2 + 2], 16).map_err(|_| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "invalid hex byte", semio_framework_diagnostic::TextSpan::at(1, (index * 2 + 1) as u32)))?;
        }
        Ok(HashProjection { latest_hash })
    }

    fn print_dsl(&self) -> String {
        let mut out = String::with_capacity(64);
        for byte in self.latest_hash {
            use std::fmt::Write;
            let _ = write!(out, "{byte:02x}");
        }
        out
    }
}

impl pack::snapshot::ArtifactPack for HashProjection {
    fn encode_pack_with(&self, _options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        Ok(self.latest_hash.to_vec())
    }
    fn decode_pack_with(bytes: &[u8], _options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let latest_hash: [u8; 32] = bytes.try_into().map_err(|_| store::PackError::from(store::PackRefusal::RetainedMalformed { kind: semio_framework_value::ValueRefusalKind::InvalidValue, what: "HashProjection Pack", offset: bytes.len() as u64, detail: "HashProjection pack must be exactly 32 bytes" }))?;
        Ok(HashProjection { latest_hash })
    }
}

impl ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot for HashProjection {
    fn reference_snapshot_codec()->Option<::semio_framework_os_kernel::os_store::serialization::ArtifactReferenceSnapshotCodec<Self>>{Some(::semio_framework_os_kernel::os_store::serialization::reference_snapshot_codec::<Self>())}


}

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
pub struct HashDiff {
    pub hash: Option<[u8; 32]>,
}

impl protocol::DiffAlgebra<HashProjection> for HashDiff {
    
    fn is_empty(&self) -> bool {
        self.hash.is_none()
    }
}

impl protocol::ColdDiffAlgebra<HashProjection> for HashDiff {
    fn inverse(&self, base: &HashProjection) -> Self {
        Self { hash: self.hash.map(|_| base.latest_hash) }
    }

    fn absorb(&mut self, other: HashDiff) {
        if other.hash.is_some() {
            self.hash = other.hash;
        }
    }


}

impl protocol::MutationDiff<HashProjection> for HashDiff {

}

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = store::__pack_json)]
pub struct HashMutation {
    pub hash: [u8; 32],
    pub author: Option<protocol::ActorId>,
    pub timestamp: Option<protocol::HybridLogicalTimestamp>,
}

const HASH_MUTATION_DESCRIPTOR: protocol::MutationLeafDescriptor = protocol::MutationLeafDescriptor {
    schema_version: 1,
    owner: "framework/os/db/tests/recovery-fixture/hash-mutation",
    semantic_kind: "set-hash",
    display_name: "Set Hash",
    emoji: "#️⃣",
    aggregate_variant: "HashMutation",
    payload_schema: "db.hash/v1",
    text_opcode: None,
    binary_tag: None,
    invertibility: protocol::MutationInvertibility::ExplicitMutation,
    diff_participation: protocol::MutationDiffParticipation::Detect,
    outcome_classes: &[protocol::MutationOutcomeClass::Applied],
    composition: protocol::MutationComposition::Atomic,
    required_language_surfaces: &[protocol::MutationLanguageSurface::Rust],
};

impl protocol::Mutation<HashProjection> for HashMutation {
    type Diff = HashDiff;
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[HASH_MUTATION_DESCRIPTOR];

    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
        &HASH_MUTATION_DESCRIPTOR
    }

    

    fn author_id(&self) -> Option<protocol::ActorId> {
        self.author.clone()
    }

    fn timestamp(&self) -> Option<protocol::HybridLogicalTimestamp> {
        self.timestamp
        }
}

impl protocol::ColdMutation<HashProjection> for HashMutation {
    type ReferenceDiff = HashDiff;

    fn diff(&self, _base: &HashProjection) -> protocol::MutationOutcome<HashDiff> {
        protocol::MutationOutcome::new(HashDiff { hash: Some(self.hash) })
    }

    /// ↩️ Restores the original hash with its author and timestamp.
    fn inverse(&self, base: &HashProjection) -> Result<Vec<HashMutation>, semio_framework_value::ValueError> {
        Ok(vec![HashMutation { hash: base.latest_hash, author: self.author.clone(), timestamp: self.timestamp }])
}
}

// 🚫️async: E1 pure accessor consumed synchronously inside `format!` — see R9
fn hex_encode(bytes: &[u8; 32]) -> String {
    use std::fmt::Write;
    let mut out = String::with_capacity(64);
    for byte in bytes {
        let _ = write!(out, "{byte:02x}");
    }
    out
}

fn hex_decode(text: &str) -> Result<[u8; 32], String> {
    if text.len() != 64 || !text.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err("expected 64 lowercase hex characters".to_string());
    }
    let mut out = [0u8; 32];
    for (index, slot) in out.iter_mut().enumerate() {
        *slot = u8::from_str_radix(&text[index * 2..index * 2 + 2], 16).map_err(|error| error.to_string())?;
    }
    Ok(out)
}

/// 🎯️ Single-line text form: `hash=<hex64>[ author=<id>][ ts=<actor>,<physical_ms>,<logical>]`.
impl protocol::OpText for HashMutation {
    fn print_op(&self) -> String {
        let mut out = format!("hash={}", hex_encode(&self.hash));
        if let Some(author) = &self.author {
            out.push_str(&format!(" author={}", author.0));
        }
        if let Some(ts) = &self.timestamp {
            out.push_str(&format!(" ts={},{},{}", ts.actor, ts.physical_ms, ts.logical));
        }
        out
    }
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let err = |detail: String| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, detail, semio_framework_diagnostic::TextSpan::at(1, 1));
        let mut hash = None;
        let mut author = None;
        let mut timestamp = None;
        for token in line.split_whitespace() {
            let (key, value) = token.split_once('=').ok_or_else(|| err(format!("malformed token '{token}'")))?;
            match key {
                "hash" => hash = Some(hex_decode(value).map_err(err)?),
                "author" => author = Some(protocol::ActorId(value.to_string().into())),
                "ts" => {
                    let parts: Vec<&str> = value.split(',').collect();
                    if parts.len() != 3 {
                        return Err(err(format!("malformed ts '{value}'")));
                    }
                    let actor = parts[0].parse::<u64>().map_err(|error| err(error.to_string()))?;
                    let physical_ms = parts[1].parse::<u64>().map_err(|error| err(error.to_string()))?;
                    let logical = parts[2].parse::<u64>().map_err(|error| err(error.to_string()))?;
                    timestamp = Some(protocol::HybridLogicalTimestamp { actor, physical_ms, logical });
                }
                other => return Err(err(format!("unknown key '{other}'"))),
            }
        }
        Ok(HashMutation { hash: hash.ok_or_else(|| err("missing hash".to_string()))?, author, timestamp })
    }
}

/// 🎯️ Binary form: `hash 32 bytes | presence u8 (bit0=author, bit1=timestamp) | [author
/// len varint + utf8 bytes] | [timestamp: actor/physical_ms/logical varint each]`.
impl protocol::OpBinary for HashMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let mut out = self.hash.to_vec();
        let presence = (self.author.is_some() as u8) | ((self.timestamp.is_some() as u8) << 1);
        out.push(presence);
        if let Some(author) = &self.author {
            pack::os_pack::write_varint_u64(&mut out, author.0.len() as u64);
            out.extend_from_slice(author.0.as_bytes());
        }
        if let Some(ts) = &self.timestamp {
            pack::os_pack::write_varint_u64(&mut out, ts.actor);
            pack::os_pack::write_varint_u64(&mut out, ts.physical_ms);
            pack::os_pack::write_varint_u64(&mut out, ts.logical);
        }
        Ok(out)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let malformed = |detail: String| protocol::ProtocolError::Malformed { what: "hash op", offset: 0, detail };
        if bytes.len() < 33 {
            return Err(malformed("truncated hash op".to_string()));
        }
        let hash: [u8; 32] = bytes[..32].try_into().expect("checked len");
        let presence = bytes[32];
        let mut pos = 33usize;
        let author = if presence & 0b01 != 0 {
            let len = pack::os_pack::read_varint_u64(bytes, &mut pos).map_err(|error| malformed(error.to_string()))? as usize;
            let end = pos + len;
            let text = std::str::from_utf8(bytes.get(pos..end).ok_or_else(|| malformed("truncated author".to_string()))?).map_err(|error| malformed(error.to_string()))?.to_string();
            pos = end;
            Some(protocol::ActorId(text.into()))
        } else {
            None
        };
        let timestamp = if presence & 0b10 != 0 {
            let actor = pack::os_pack::read_varint_u64(bytes, &mut pos).map_err(|error| malformed(error.to_string()))?;
            let physical_ms = pack::os_pack::read_varint_u64(bytes, &mut pos).map_err(|error| malformed(error.to_string()))?;
            let logical = pack::os_pack::read_varint_u64(bytes, &mut pos).map_err(|error| malformed(error.to_string()))?;
            Some(protocol::HybridLogicalTimestamp { actor, physical_ms, logical })
        } else {
            None
        };
        Ok(HashMutation { hash, author, timestamp })
    }
}

struct HashOwnedRetirement<T>(Option<T>);

impl<T: Send> store::ErasedSnapshotRetirement for HashOwnedRetirement<T> {
    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, semio_framework_value::ValueError> {
        if maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.0.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.0.is_none()
    }
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
struct HashOwnedRetirementFactory;

impl<T: Send + 'static> store::ArtifactOwnedValueRetirementFactory<T> for HashOwnedRetirementFactory {
    fn retire_owned(&self, value: T) -> Box<dyn store::ErasedSnapshotRetirement> {
        Box::new(HashOwnedRetirement(Some(value)))
    }
}

struct HashSnapshotRetirement(Option<Arc<HashProjection>>);

impl store::ErasedSnapshotRetirement for HashSnapshotRetirement {
    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, semio_framework_value::ValueError> {
        if maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.0.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.0.is_none()
    }
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
struct HashSnapshotRetirementFactory;

impl store::SnapshotRetirementFactory<HashProjection> for HashSnapshotRetirementFactory {
    fn retirement_birth_bytes(&self, _snapshot: &Arc<HashProjection>) -> usize { std::mem::size_of::<HashSnapshotRetirement>() }

    fn retire(&self, snapshot: Arc<HashProjection>) -> Box<dyn store::ErasedSnapshotRetirement> {
        Box::new(HashSnapshotRetirement(Some(snapshot)))
    }
}

impl store::os_store::document_store_owners::MemberStoreProducer<HashProjection,HashMutation> for HashMemberStoreProducer {
    /// 📦️ The fixture projection opens as an owned member through its own `ArtifactPack` codec.
    type SnapshotOpen = store::ProducedMemberSnapshotOpen<HashProjection,store::ArtifactPackSnapshotDecoder<HashProjection>>;

    fn member_store_owners_birth_demand(&self) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, semio_framework_value::ValueError> {
        Ok(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: store::os_store::document_store_owners::DocumentStoreOwners::<HashProjection, HashMutation>::source_birth_bytes::<HashSnapshotRetirementFactory, HashOwnedRetirementFactory, HashOwnedRetirementFactory, store::ArtifactStoreCursorDisposer<HashProjection, HashMutation>>()?, depth: 1 })
    }

    fn member_store_owners(&self,grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<(store::os_store::document_store_owners::DocumentStoreOwners<HashProjection, HashMutation>, semio_framework_value::retained_clone::RetainedCloneProgress), store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<HashProjection, HashMutation>> {
        store::os_store::document_store_owners::DocumentStoreOwners::admit_source_constructor(grant, || (HashSnapshotRetirementFactory, HashOwnedRetirementFactory, HashOwnedRetirementFactory, store::ArtifactStoreCursorDisposer::<HashProjection, HashMutation>::new()))
    }
}
//#endregion 🔖️RecoveryFixtureStore

async fn committed_recovery_hash_store(id: &str, dialect: semio_framework_artifact_reference::ArtifactDialect, owner: Option<store::OwnerRef>) -> store::ArtifactStore<HashProjection, HashMutation> {
    use store::os_store::document_store_owners::MemberStoreProducer as _;
    let mut envelope = store::os_store::document_materialize::create_document_envelope::<HashProjection, HashMutation>("db.hash/v1", id, HashProjection::default(), None, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    envelope.dialect = Some(dialect);
    envelope.owner = owner;
    let mut store = store::ArtifactStore::new::<::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication>(envelope, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await.expect("committed recovery fixture creates one exact Store");
    store.install_document_store_owners_exact(store::os_store::document_store_owners::funded_member_store_owners(&HashMemberStoreProducer).expect("actual cold fixture catalog source birth"));
    store
}

fn close_committed_recovery_hash_store(store: &mut store::ArtifactStore<HashProjection, HashMutation>) {
        let grant = semio_framework_value::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 4_096, maximum_capacity_bytes: 4_096, maximum_release_bytes: 4_096, maximum_depth: 64 };
for _ in 0..4_096 {
        let step = store::SpaceMember::close_owned_step(store, grant).expect("committed recovery fixture Store closes");
        assert!(step.progress().fits(grant));
        if matches!(step, semio_framework_value::RetainedCloneStep::Complete(_)) {
            assert!(store::SpaceMember::close_owned_terminal_is_empty(store));
            return;
        }
    }
    panic!("committed recovery fixture Store did not reach terminal handoff");
}

fn close_journal_commit(commit: &mut dyn store::durable_group::DurableOwnedGroupJournalCommitV1) {
    commit.begin_close();
    let grant = semio_framework_value::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 4096, maximum_capacity_bytes: store::durable_group::DURABLE_OWNED_GROUP_STRUCTURAL_MAX_BYTES, maximum_release_bytes: store::durable_group::DURABLE_OWNED_GROUP_EVENT_MAX_BYTES, maximum_depth: 64 };
    for _ in 0..8 {
        let step = commit.close_step(grant).unwrap();
        assert!(step.progress().fits(grant));
        if matches!(step, semio_framework_value::RetainedCloneStep::Complete(_)) {
            assert!(commit.terminal_is_empty());
            return;
        }
    }
    panic!("journal commit did not reach its terminal-empty witness");
}

async fn shutdown_journal_authority(authority: &ArtifactAuthority, pool: &semio_framework_async::WorkerPool) {
    let mut terminal_job = None;
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    while std::time::Instant::now() < deadline {
        if authority.shutdown_step() {
            pool.shutdown();
            return;
        }
        if terminal_job.is_none() {
            terminal_job = authority.take_terminal_job();
        }
        if let Some(job) = terminal_job.take() {
            if let Err(retained) = job.close() {
                terminal_job = Some(retained);
            }
        }
        semio_framework_async::yield_once().await;
    }
    panic!("journal authority did not terminate: {}", authority.shutdown_debug_witness());
}

async fn durable_group_witness_batch(kinds: &[serde_json::Value], canonical_pack: &[u8]) -> db_wal_log::WalRecordBatch {
    let mut control = db_wal_log::WalCursorControl::new(StdArc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 1_000_000).unwrap();
    let mut records = db_wal_log::WalRecordBatch::new();
    for kind in kinds {
        let bytes = admit_wal_bytes(canonical_pack.to_vec(), db_io_core::DB_IO_MAX_READ_BYTES, &mut control).await.unwrap();
        let record = match kind.as_str().unwrap() {
            "event" => db_wal_log::WalRecord::Event(bytes),
            "command" => db_wal_log::WalRecord::Command(bytes),
            other => panic!("unknown durable witness record kind {other}"),
        };
        push_wal_record(&mut records, record, &mut control).await.unwrap();
    }
    records
}
//#endregion 🔖️Actor

impl protocol::ColdMutationDiff<HashProjection> for HashDiff {
    fn apply(&self, base: &mut HashProjection, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        if let Some(hash) = self.hash { base.latest_hash = hash; }
        Ok(())
    }
}
