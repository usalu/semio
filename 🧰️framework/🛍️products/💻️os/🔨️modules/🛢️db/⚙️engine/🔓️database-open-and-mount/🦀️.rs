//! 🔓️ Database open and mount: the open operations, the catalog read and bootstrap retained opens, the document mount and its adoption over the emit runtime. See [`super`].

use super::*;

//#region 🔖️Database
// 🔀️ dedyn-emit-runtime, O1/R11(a): every method below reads/writes `self.emit`, so this whole
// block is generic over `E: Emit`; Rust resolves `open`'s `Database::open_with(..)` call by inferring
// `E` from `default_emit()`'s concrete return type.
impl<E: Emit + 'static> Database<E> {
    pub(super) fn require_open_use(&self) -> Result<Arc<WorkerPoolUse>, DbError> {
        if self.shutdown_complete {
            return Err(DbError::Closed);
        }
        self.pool_use.clone().ok_or(DbError::Closed)
    }

    /// 🧵️ Admits the exact storage owner before probing one backend capability future on I/O.
    pub fn open_retained(pool: Arc<WorkerPool>, storage: Arc<db_storage::DbBackend>) -> Result<DatabaseCapabilityOpenFuture, DatabaseCapabilityOpenRejected> {
        DatabaseCapabilityOpenFuture::try_submit(pool, storage)
    }

    /// 🗂️ Admits the exact storage/root-key owners before one catalog backend read on I/O.
    pub fn open_catalog_read_retained(pool: Arc<WorkerPool>, storage: Arc<db_storage::DbBackend>) -> Result<DatabaseCatalogReadFuture, DatabaseCatalogReadRejected> {
        DatabaseCatalogReadFuture::try_submit(pool, storage, DatabaseCatalogRootKey::root())
    }

    /// 🌱️ Admits the exact storage/catalog-page/fence owners before the initial CAS reaches I/O.
    pub fn open_catalog_bootstrap_retained(pool: Arc<WorkerPool>, storage: Arc<db_storage::DbBackend>, pages: db_io_core::DbIoPages) -> Result<DatabaseCatalogBootstrapFuture, DatabaseCatalogBootstrapRejected> {
        DatabaseCatalogBootstrapFuture::try_submit(pool, storage, pages, EpochFence::INITIAL)
    }

    pub(super) async fn open_with(pool: Arc<WorkerPool>, config: DbConfig, storage: Arc<db_storage::DbBackend>, emit: Arc<E>) -> Result<Database<E>, DbError> {
        let pool_use = pool.acquire_use().map_err(|error| DbError::Unavailable(format!("database WorkerPool use rejected: {error:?}")))?;
        let capability_probe = match DatabaseCapabilityOpenFuture::try_prepare_with_use(pool.clone(), pool_use.clone(), storage, true) {
            Ok(probe) => probe,
            Err(rejected) => return Err(rejected.close_and_take_error()),
        };
        let (storage, storage_capabilities) = capability_probe.await?.into_parts();
        let capabilities = DbCapabilities {
            preview: false,
            historical_query: true,
            live_query: false,
            cluster: false,
            max_durability: std::cmp::min(storage_capabilities.max_durability, config.capabilities.max_durability),
        };

        let health = Arc::new(db_observe::HealthRegistry::new());
        health.set("db_engine.storage", if storage_capabilities.durable { db_observe::HealthState::Healthy } else { db_observe::HealthState::Degraded("storage backend is not durable".to_string()) });

        let catalog_probe = match DatabaseCatalogReadFuture::try_prepare_with_use(pool.clone(), pool_use.clone(), storage, DatabaseCatalogRootKey::root(), true) {
            Ok(probe) => probe,
            Err(rejected) => return Err(rejected.close_and_take_error(pool.clone())),
        };
        let (storage, _catalog_key, catalog_root) = catalog_probe.await?.into_parts();
        let (storage, epoch, entries) = match catalog_root? {
            Some((bytes, epoch)) => {
                let prepared = db_io_registry::db_io_prepare_platform(&bytes)?.await?;
                (storage, epoch, decode_catalog(prepared.as_slice()).await?)
            }
            None => {
                let pages = encode_catalog_pages(&[]).await?;
                let bootstrap = match DatabaseCatalogBootstrapFuture::try_prepare_with_use(pool.clone(), pool_use.clone(), storage, pages, DatabaseCatalogBootstrapKey::root(), EpochFence::INITIAL, true) {
                    Ok(bootstrap) => bootstrap,
                    Err(rejected) => return Err(rejected.close_and_take_error()),
                };
                let result = bootstrap.await?;
                let (retained_storage, _catalog_key, expected, actual) = match result.into_parts() {
                    Ok(parts) => parts,
                    Err(owner) => {
                        drop(owner);
                        return Err(DbError::Internal("database catalog-bootstrap result was incomplete".to_string()));
                    }
                };
                if expected != EpochFence::INITIAL {
                    return Err(DbError::Internal("database catalog-bootstrap returned the wrong expected fence".to_string()));
                }
                let epoch = actual?;
                (retained_storage, epoch, Vec::new())
            }
        };
        health.set("db_engine.catalog", db_observe::HealthState::Healthy);

        emit.emit(EmitEvent::new("db_engine.database_opened").field("documents", EmitField::U64(entries.len() as u64))).await;

        Ok(Database {
            storage,
            config,
            capabilities,
            emit,
            health,
            catalog: Arc::new(Mutex::new(CatalogState { epoch, revision: 1, entries: Arc::new(entries), pending: None })),
            open_artifacts: Arc::new(Mutex::new(DatabaseDocumentMountRegistry::new())),
            #[cfg(test)]
            mount_catalog_published_hook: Arc::new(Mutex::new(None)),
            #[cfg(test)]
            mount_cleanup_fault_hook: Arc::new(Mutex::new(None)),
            #[cfg(test)]
            mount_parked_hook: Arc::new(Mutex::new(None)),
            closing_authority: None,
            shutdown_emit_started: false,
            shutdown_complete: false,
            pool_use: Some(pool_use),
            pool,
        })
    }

    /// ⚙️ Builds one `ArtifactEngineConfig`: this database's own `limits` and preview TTL, and
    /// `..db_artifact::ArtifactEngineConfig::default()` for every other field (`security`/`emit`/
    /// `projections`) — this crate has no opinion on those (`db_artifact`'s own real
    /// `db_security::SecurityGate`-backed default policy is permissive single-tenant), and the spread keeps
    /// this call site correct across further additive growth without another coordinated edit.
    fn document_engine_config(&self) -> db_artifact_engine::ArtifactEngineConfig {
        db_artifact_engine::ArtifactEngineConfig { limits: self.config.limits.clone(), preview_ttl_ms: self.config.limits.max_preview_ttl_ms, ..db_artifact_engine::ArtifactEngineConfig::default() }
    }

    fn retained_mount_rejection(rejected: db_artifact_engine::ArtifactEngineOpenRejected, resume: Option<DatabaseDocumentMountFuture>) -> DatabaseDocumentMountFailure {
        DatabaseDocumentMountFailure::Retained { rejected, resume }
    }

    async fn refresh_catalog_document(pool: Arc<WorkerPool>, storage: Arc<db_storage::DbBackend>, catalog: Arc<Mutex<CatalogState>>, document: &protocol::ArtifactId) -> Result<bool, DbError> {
        let read = match DatabaseCatalogReadFuture::try_submit(pool.clone(), storage, DatabaseCatalogRootKey::root()) {
            Ok(read) => read,
            Err(rejected) => return Err(rejected.close_and_take_error(pool)),
        };
        let result = read.await?;
        let (_storage, _key, root) = result.into_parts();
        let Some((bytes, epoch)) = root? else { return Ok(false) };
        let prepared = db_io_registry::db_io_prepare_platform(&bytes)?.await?;
        let entries = decode_catalog(prepared.as_slice()).await?;
        let known = entries.iter().any(|entry| entry.document == *document);
        let mut state = catalog.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if epoch.epoch >= state.epoch.epoch {
            state.epoch = epoch;
            state.revision = state.revision.checked_add(1).ok_or(DbError::LimitExceeded("database catalog refresh revision"))?;
            state.entries = Arc::new(entries);
            state.pending = None;
        }
        Ok(known)
    }

    async fn publish_mount_catalog(_live: DatabaseMountFutureLiveGuardV1, pool: Arc<WorkerPool>, pool_use: Arc<WorkerPoolUse>, storage: Arc<db_storage::DbBackend>, catalog: Arc<Mutex<CatalogState>>, document: protocol::ArtifactId) -> Result<(), DbError> {
        let transaction = match DatabaseCreateCatalogFuture::try_prepare_with_use(pool, pool_use, catalog, storage, document, true) {
            Ok(transaction) => transaction,
            Err(rejected) => return Err(rejected.close_and_take_error()),
        };
        let result = transaction.await?;
        let (_storage, _document, _expected, actual) = result.into_parts().map_err(|_| DbError::LimitExceeded("database create-catalog result owner"))?;
        actual.map(|_| ())
    }

    async fn run_open_document_mount(
        _live: DatabaseMountFutureLiveGuardV1,
        pool: Arc<WorkerPool>,
        pool_use: Arc<WorkerPoolUse>,
        storage: Arc<db_storage::DbBackend>,
        document: protocol::ArtifactId,
        config: db_artifact_engine::ArtifactEngineConfig,
        mailbox_capacities: MailboxCapacities,
        emit: Arc<E>,
    ) -> Result<DatabaseDocumentMountReply, DatabaseDocumentMountFailure> {
        let open_document = document.clone();
        let opened_at_ms = now_ms().await;
        let authority = match db_artifact::ArtifactAuthority::spawn_with_pool_use(
            pool,
            pool_use,
            move || async move { db_artifact_engine::ArtifactEngine::open_retained(open_document, storage, config, opened_at_ms).await.map(|(engine, _)| Box::new(engine)) },
            mailbox_capacities,
        )
        .await
        {
            Ok(authority) => authority,
            Err(rejected) => return Err(Self::retained_mount_rejection(rejected, None)),
        };
        emit.emit(EmitEvent::new("db_engine.document_opened").with_document(to_core_document_id(&document).await)).await;
        Ok(DatabaseDocumentMountReply { authority: Arc::new(authority), handles: Arc::new(std::sync::atomic::AtomicUsize::new(0)) })
    }

    async fn run_document_mount(
        _live: DatabaseMountFutureLiveGuardV1,
        predecessor: Option<db_actor::ReplyReceiver<()>>,
        pool: Arc<WorkerPool>,
        pool_use: Arc<WorkerPoolUse>,
        storage: Arc<db_storage::DbBackend>,
        catalog: Arc<Mutex<CatalogState>>,
        document: protocol::ArtifactId,
        policy: DatabaseDocumentMountPolicy,
        catalog_known: bool,
        create_config: db_artifact_engine::ArtifactEngineConfig,
        open_config: db_artifact_engine::ArtifactEngineConfig,
        mailbox_capacities: MailboxCapacities,
        emit: Arc<E>,
        #[cfg(test)] mount_catalog_published_hook: Arc<Mutex<Option<DatabaseDocumentMountPublishedHook>>>,
    ) -> Result<DatabaseDocumentMountReply, DatabaseDocumentMountFailure> {
        if let Some(predecessor) = predecessor {
            let _ = predecessor.await;
        }
        let mut create = !catalog_known && policy != DatabaseDocumentMountPolicy::Open;
        if create {
            if let Err(error) = Self::publish_mount_catalog(DatabaseMountFutureLiveGuardV1::new(), pool.clone(), pool_use.clone(), storage.clone(), catalog.clone(), document.clone()).await {
                if policy != DatabaseDocumentMountPolicy::Ensure || !matches!(error, DbError::AlreadyExists(_) | DbError::Fenced { .. }) {
                    return Err(error.into());
                }
                if !Self::refresh_catalog_document(pool.clone(), storage.clone(), catalog.clone(), &document).await? {
                    return Err(error.into());
                }
                create = false;
            }
            #[cfg(test)]
            if create {
                if let Some(hook) = mount_catalog_published_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
                    hook(&document);
                }
            }
        }

        let authority = if create {
            let create_pool = pool.clone();
            let create_storage = storage.clone();
            let create_document = document.clone();
            let created_at_ms = now_ms().await;
            match db_artifact::ArtifactAuthority::spawn_with_pool_use(create_pool, pool_use.clone(), move || async move { db_artifact_engine::ArtifactEngine::create_retained(create_document, create_storage, create_config, created_at_ms).await.map(Box::new) }, mailbox_capacities).await {
                Ok(authority) => authority,
                Err(rejected) => {
                    let retry_open = policy == DatabaseDocumentMountPolicy::Ensure && matches!(rejected.error(), DbError::AlreadyExists(_));
                    let resume = retry_open.then(|| Box::pin(Self::run_open_document_mount(DatabaseMountFutureLiveGuardV1::new(), pool.clone(), pool_use.clone(), storage.clone(), document.clone(), open_config, mailbox_capacities, emit.clone())) as DatabaseDocumentMountFuture);
                    return Err(Self::retained_mount_rejection(rejected, resume));
                }
            }
        } else {
            return Self::run_open_document_mount(DatabaseMountFutureLiveGuardV1::new(), pool, pool_use, storage, document, open_config, mailbox_capacities, emit).await;
        };
        emit.emit(EmitEvent::new("db_engine.document_created").with_document(to_core_document_id(&document).await)).await;
        Ok(DatabaseDocumentMountReply { authority: Arc::new(authority), handles: Arc::new(std::sync::atomic::AtomicUsize::new(0)) })
    }

    /// 🔁️ Mounts until a handle is adopted. A mount whose document went idle and started closing
    /// between its reply and its adoption mounts again, then as an open of the document it created.
    pub(super) async fn mount_document(&self, document: protocol::ArtifactId, policy: DatabaseDocumentMountPolicy) -> Result<ArtifactHandle, DatabaseDocumentOpenRejected> {
        let mut policy = policy;
        loop {
            if let Some(handle) = self.mount_document_once(document.clone(), policy).await? {
                return Ok(handle);
            }
            if policy == DatabaseDocumentMountPolicy::Create {
                policy = DatabaseDocumentMountPolicy::Open;
            }
        }
    }

    fn adopt_mount(&self, document: &protocol::ArtifactId, handles: &Arc<std::sync::atomic::AtomicUsize>, registry: &DatabaseDocumentMountRegistry) -> Option<ArtifactHandleLease> {
        match registry.slots.get(&document.0) {
            Some(DatabaseDocumentMountSlot::Ready(ready)) if Arc::ptr_eq(&ready.handles, handles) => Some(ArtifactHandleLease::new(handles.clone(), Arc::downgrade(&self.open_artifacts), document.0.clone())),
            _ => None,
        }
    }

    async fn mount_document_once(&self, document: protocol::ArtifactId, policy: DatabaseDocumentMountPolicy) -> Result<Option<ArtifactHandle>, DatabaseDocumentOpenRejected> {
        let pool_use = self.require_open_use()?;
        let _pool_use_site = DatabasePoolUseSiteGuardV1::new(&DATABASE_POOL_USE_MOUNT_WAIT_LIVE);
        let catalog_known = self.catalog.lock().expect("db_engine: catalog mutex poisoned").entries.iter().any(|entry| entry.document == document);
        let create_config = self.document_engine_config();
        let open_config = self.document_engine_config();
        let (reply, owner) = {
            let mut registry = self.open_artifacts.lock().expect("db_engine: open_artifacts mutex poisoned");
            if let Some(DatabaseDocumentMountSlot::Ready(completion)) = registry.slots.get(&document.0) {
                if policy == DatabaseDocumentMountPolicy::Create {
                    return Err(DbError::AlreadyExists(format!("document {} already exists", document.0)).into());
                }
                let completion = completion.clone();
                let lease = self.adopt_mount(&document, &completion.handles, &registry);
                drop(registry);
                return Ok(lease.map(|lease| ArtifactHandle { authority: completion.authority, _lease: lease, document, pool: self.pool.clone() }));
            }
            if let Some(DatabaseDocumentMountSlot::Opening { generation, owner, waiters }) = registry.slots.get_mut(&document.0) {
                let Some(slot) = waiters.iter().position(Option::is_none) else {
                    return Err(DbError::Unavailable("database document-mount waiter capacity exhausted".to_string()).into());
                };
                let generation = *generation;
                let (reply, raw_receiver) = db_actor::oneshot();
                waiters[slot] = Some(DatabaseDocumentMountWaiter { policy, create_claim: false, reply });
                (DatabaseDocumentMountWait { receiver: raw_receiver, registry: Arc::downgrade(&self.open_artifacts), document: document.0.clone(), generation, slot, resolved: false }, Some(owner.clone()))
            } else {
                if policy == DatabaseDocumentMountPolicy::Create && catalog_known {
                    return Err(DbError::AlreadyExists(format!("document {} already exists", document.0)).into());
                }
                if policy == DatabaseDocumentMountPolicy::Open && !catalog_known {
                    return Err(DbError::NotFound(format!("document {} not found", document.0)).into());
                }
                let generation = registry.take_generation()?;
                let predecessor = match registry.slots.remove(&document.0) {
                    Some(DatabaseDocumentMountSlot::Closing { terminal }) => Some(terminal),
                    Some(other) => {
                        registry.slots.insert(document.0.clone(), other);
                        return Err(DbError::Internal("database document-mount slot changed under its lock".to_string()).into());
                    }
                    None => None,
                };
                let (reply, raw_receiver) = db_actor::oneshot();
                let future = Box::pin(Self::run_document_mount(
                    DatabaseMountFutureLiveGuardV1::new(),
                    predecessor,
                    self.pool.clone(),
                    pool_use,
                    self.storage.clone(),
                    self.catalog.clone(),
                    document.clone(),
                    policy,
                    catalog_known,
                    create_config,
                    open_config,
                    self.config.mailbox_capacities,
                    self.emit.clone(),
                    #[cfg(test)]
                    self.mount_catalog_published_hook.clone(),
                ));
                let owner = DatabaseDocumentMountOwner::new(
                    self.pool.clone(),
                    Arc::downgrade(&self.open_artifacts),
                    document.0.clone(),
                    generation,
                    future,
                    #[cfg(test)]
                    self.mount_cleanup_fault_hook.clone(),
                    #[cfg(test)]
                    self.mount_parked_hook.clone(),
                );
                let mut waiters = std::array::from_fn(|_| None);
                waiters[0] = Some(DatabaseDocumentMountWaiter { policy, create_claim: policy == DatabaseDocumentMountPolicy::Create, reply });
                registry.slots.insert(document.0.clone(), DatabaseDocumentMountSlot::Opening { generation, owner: owner.clone(), waiters });
                (DatabaseDocumentMountWait { receiver: raw_receiver, registry: Arc::downgrade(&self.open_artifacts), document: document.0.clone(), generation, slot: 0, resolved: false }, Some(owner))
            }
        };
        if let Some(owner) = owner {
            owner.request_drive(false);
        }
        let result = reply.await.map_err(DatabaseDocumentOpenRejected::Database)?;
        let result = result.map_err(DatabaseDocumentOpenRejected::Database)?;
        let lease = {
            let registry = self.open_artifacts.lock().expect("db_engine: open_artifacts mutex poisoned");
            self.adopt_mount(&document, &result.handles, &registry)
        };
        Ok(lease.map(|lease| ArtifactHandle { authority: result.authority, _lease: lease, document, pool: self.pool.clone() }))
    }

}
//#endregion 🔖️Database
