//! 🏗️ Open and discovery of the headless workspace: construction, the folder and hub openers, the origin, catalog discovery, artifact byte reads and the settled hub snapshots. See [`super`].

use super::*;

impl HeadlessWorkspace {
    pub fn new(origin: WorkspaceOrigin, principal: String, scopes: Vec<String>, catalog: Arc<Catalog>, host_driver_policy:semio_framework_plugin_host::GuestRelayDriverPolicy) -> Self {
        ensure_probe_codec_registered();
        let session_id = mint_session_id(&principal, 0);
        Self {
            #[cfg(test)]
            test_discovery:None,
            host_driver_policy,
            artifact_host: store::sync::ArtifactHost::new(workspace_worker_pool()),
            origin,
            principal,
            session_id,
            scopes,
            repo_root: find_repo_root().ok(),
            catalog: Arc::new(WorkspaceCatalog::fixed(catalog)),
            open_probes: Mutex::new(HashMap::new()),
            action_adapter: Mutex::new(None),
            #[cfg(not(target_arch = "wasm32"))]
            root_actions: OnceLock::new(),
            plugin_artifacts: Arc::new(Mutex::new(HashMap::new())),
            shell_route: OnceLock::new(),
            hub_binding: None,
            #[cfg(not(target_arch = "wasm32"))]
            hub_driver: None,
            #[cfg(not(target_arch = "wasm32"))]
            plugin_components: find_repo_root().ok().map(PluginComponentSource::Repo),
        }
    }

    /// 🚫 The git repository root is not a workspace folder. Binding it persists `.semio/events.semio` in the tree.
    pub fn reject_repository_root_workspace(path: &std::path::Path) -> Result<(), GatewayError> {
        if is_repository_root(path).map_err(|error| GatewayError::new(GatewayErrorCode::InputInvalid, error))? {
            return Err(GatewayError::new(GatewayErrorCode::InputInvalid, format!("--folder `{}` is the repository root and would write .semio/events.semio into the git tree; pass a directory that is not the repository root", path.display())));
        }
        Ok(())
    }

    pub fn open_folder(path: PathBuf, principal: String, scopes: Vec<String>, catalog: Arc<Catalog>, host_driver_policy:semio_framework_plugin_host::GuestRelayDriverPolicy) -> Result<Self, GatewayError> {
        if !path.is_dir() {
            std::fs::create_dir_all(&path).map_err(|error| GatewayError::new(GatewayErrorCode::InputInvalid, format!("--folder `{}` does not exist and could not be created: {error}", path.display())))?;
        }
        Self::reject_repository_root_workspace(&path)?;
        Ok(Self::new(WorkspaceOrigin::Folder { path }, principal, scopes, catalog, host_driver_policy))
    }

    /// 🧩️ A hub-bound workspace runs the HUB's components, never a build tree that happens to
    /// sit on the same disk: the hub is the authority over what code opens its documents.
    pub fn open_hub(base_url: String, space_id: String, credential: Arc<LocalHubCredential>, principal: String, scopes: Vec<String>, host_driver_policy:semio_framework_plugin_host::GuestRelayDriverPolicy) -> Result<Self, GatewayError> {
        remote::validate_hub_origin(&base_url, &space_id)?;
        #[cfg(not(target_arch = "wasm32"))]
        {
            let (binding, driver, grant_source) = NativeHubBindingDriver::connect(credential.clone(), &base_url, &space_id)?;
            binding.await_settled(remote::HUB_AUTHORITY_SETTLE_WAIT_MS);
            let catalog = WorkspaceCatalog::following(Arc::clone(&binding))?;
            let driver = Arc::new(driver);
            let mut workspace = Self::new(WorkspaceOrigin::Hub { base_url, space_id }, principal, scopes, catalog.current(), host_driver_policy);
            workspace.catalog = Arc::new(catalog);
            workspace.repo_root = None;
            workspace.plugin_components = Some(PluginComponentSource::Hub(Arc::new(HubPluginComponents::new(Arc::clone(&binding), Arc::clone(&driver)))));
            workspace.artifact_host.set_local_hub_credential(credential);
            workspace.artifact_host.set_hub_socket_grant_source(grant_source);
            workspace.hub_binding = Some(binding);
            workspace.hub_driver = Some(driver);
            Ok(workspace)
        }
        #[cfg(target_arch = "wasm32")]
        {
            let _ = (base_url, space_id, credential, principal, scopes);
            Err(GatewayError::new(GatewayErrorCode::PluginUnavailable, "native hub directory transport is unavailable on wasm32").retryable())
        }
    }

    pub fn origin(&self) -> &WorkspaceOrigin {
        &self.origin
    }

    /// 🧩️ This workspace's plugin component source — the repo's own build tree for `--folder`, the
    /// hub's authorized execution targets for `--hub`.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn plugin_components(&self) -> Option<PluginComponentSource> {
        self.plugin_components.clone()
    }

    /// 🔐 The live Hub-selected package roster. Folder workspaces have no Hub authority and callers
    /// must use installed discovery there instead.
    pub fn verified_hub_catalog_selections(&self) -> Result<Arc<AuthorizedCatalogSnapshot>, GatewayError> {
        match &self.origin {
            WorkspaceOrigin::Hub { .. } => self.settled_hub_binding()?.ready_catalog_snapshot(i64::try_from(now_ms()).unwrap_or(i64::MAX)),
            WorkspaceOrigin::Folder { .. } => Err(GatewayError::new(GatewayErrorCode::InputInvalid, "folder workspaces do not have a Hub-selected catalog")),
        }
    }

    /// 📚 Package descriptors used for capability and inference discovery. Hub mode reads only the
    /// authenticated retained snapshot; folder mode may use the installed registry.
    pub fn discovery_descriptors(&self) -> Result<Vec<semio_framework::PackageDescriptor>, GatewayError> {
        #[cfg(test)]
        if let Some(descriptors)=&self.test_discovery {let owners=self.catalog_plugin_ids();return Ok(descriptors.iter().filter(|entry|owners.contains(&entry.manifest.plugin_id)).cloned().collect());}
        match &self.origin {
            WorkspaceOrigin::Hub { .. } => Ok(self.verified_hub_catalog_selections()?.selections.iter().map(|selection| selection.descriptor.clone()).collect()),
            WorkspaceOrigin::Folder { .. } => {
                let repo_root = find_repo_root()?;
                let registry = load_plugin_registry(&repo_root)?;
                let mut descriptors = Vec::new();
                for plugin_id in self.catalog_plugin_ids() {
                    descriptors.push(load_package_descriptor(&find_plugin_entry(&registry, &plugin_id)?.owner_root)?);
                }
                Ok(descriptors)
            }
        }
    }

    /// 🗂️ Recompiles a discovery catalog from the current authoritative descriptor set. Revoked or
    /// refreshing Hub authority returns unavailable and never consults a local registry.
    pub fn discovery_catalog(&self) -> Result<Arc<Catalog>, GatewayError> {
        match &self.origin {
            WorkspaceOrigin::Hub { .. } => self.catalog.authoritative(),
            WorkspaceOrigin::Folder { .. } => Ok(self.catalog.current()),
        }
    }

    /// 🗿️ A hub document of another schema is not an ERROR, it is simply not a probe document.
    /// This used to answer a retryable `PLUGIN_UNAVAILABLE` naming the probe schema, which made
    /// `artifact_open` of EVERY real hub document fail before it ever reached
    /// [`Self::read_artifact_bytes`] — measured live on hub 7631 against a `gis.map` document
    /// (ticket 26/09/18 slice M8, `📓️m8-mcp-agent-third-participant.md` §2.3 row 4). The
    /// predicate's own name is the contract: it answers whether the probe lane owns this id.
    pub fn authenticated_probe_document_is_known(&self, artifact_id: &str) -> Result<bool, GatewayError> {
        if !matches!(self.origin, WorkspaceOrigin::Hub { .. }) {
            return Ok(false);
        }
        let snapshot = self.hub_snapshot()?;
        let Some(document) = snapshot.documents.values().find(|document| document.scope.document_id == artifact_id) else {
            return Ok(false);
        };
        Ok(document.view.descriptor.artifact_schema == PROBE_SCHEMA && document.view.descriptor.pack_schema_hash == PROBE_PACK_SCHEMA_HASH)
    }

    pub(crate) fn actor_label(&self) -> String {
        format!("agent:{}#{}", self.principal, self.session_id)
    }

    /// 🗂️ Every document id currently known to this workspace: the union of (a) every id this
    /// workspace has itself opened a live `ProbeStore` for (`self.open_probes` — populated
    /// SYNCHRONOUSLY by `ensure_probe_artifact`, before this call can ever see it) and (b) whatever
    /// is already durably persisted on disk (`FolderEventLogStorage`'s document events).
    ///
    /// 🪲️ Post-unblock fix (see `📓️terra-P7-report.md`'s "## post-unblock fixes"): this used to
    /// consult ONLY (b). `ArtifactHost::open`'s actor persists asynchronously on its own thread —
    /// `ensure_probe_artifact`'s `dispatch`+`send` returns as soon as the LOCAL `ArtifactStore` has
    /// applied the mutation, before the actor thread has necessarily flushed it to
    /// `<folder>/.semio/documents.db`. A cold-disk-only reader therefore raced the actor and lost:
    /// exactly `resolve_context_reports_the_open_probe_artifact_as_active`'s failure. (a) closes that
    /// gap because it is the SAME map `ensure_probe_artifact` writes to, read under the same lock,
    /// with no cross-thread hop in between.
    pub fn workspace_artifact_ids(&self) -> Result<Vec<String>, GatewayError> {
        match &self.origin {
            WorkspaceOrigin::Folder { path } => {
                let mut ids: std::collections::BTreeSet<String> = self.open_probes.lock().unwrap_or_else(std::sync::PoisonError::into_inner).keys().cloned().collect();
                let storage = store::sync::FolderEventLogStorage::new(path.clone());
                let persisted = ::semio_framework_async::poll::resolve_ready(storage.document_ids()).map_err(|error| GatewayError::new(GatewayErrorCode::Internal, format!("listing {}: {error}", path.display())))?;
                ids.extend(persisted);
                Ok(ids.into_iter().collect())
            }
            WorkspaceOrigin::Hub { .. } => {
                let snapshot = self.hub_snapshot()?;
                let mut ids: Vec<_> = snapshot.documents.keys().map(|scope| scope.document_id.clone()).collect();
                ids.sort();
                Ok(ids)
            }
        }
    }

    /// 📖️ One document's real pack+spr bytes, read from whoever OWNS that document right now:
    /// the LIVE in-memory `ProbeStore` snapshot (`ArtifactStore::snapshot_pack`, real bytes, no
    /// cross-thread wait) when this workspace has one open for `artifact_id`; the session's own
    /// channel when this session resolved `shell` and `artifact_id` is one this gateway bound to a
    /// plugin ([`read_session_artifact_bytes`] — the store the human is editing); else a cold
    /// `FolderEventLogStorage` read (host-opaque either way — no codec, no interpretation). `None`
    /// for a hub binding with no live store open, or a folder id with neither an open store nor a
    /// persisted row. Same race fix as `workspace_artifact_ids` — see that method's own doc.
    ///
    /// 🧊️ The middle lane is the law `artifact_snapshot` was missing: `create_plugin_artifact`
    /// seeds the folder event log from ONE `ReadArtifact` at create time, so every later read of
    /// that row answers the bytes as they were when the agent opened the document. Measured
    /// 2026-09-20 (AP1, `🗑️generated/ap1-shell-route-probe.txt`): an `addBlock` invocation moved the
    /// live head from `cursor 0` to `cursor 1, headEditId "apply"` and `artifact_snapshot` answered
    /// a byte-identical 688-char pack on both sides of it. A snapshot that cannot show the change
    /// the agent just made is worse than no snapshot, so a session-owned document is re-read from
    /// its owner and the frozen row is never consulted for it.
    ///
    /// 🧊️ Headless lane: the live guest instance the mutation went to, never the row frozen
    /// at create time — see [`Self::read_live_session_artifact_bytes`].
    pub fn read_artifact_bytes(&self, artifact_id: &str) -> Result<Option<(Vec<u8>, Vec<u8>)>, GatewayError> {
        if let Some(probe_store) = self.open_probes.lock().unwrap_or_else(std::sync::PoisonError::into_inner).get(artifact_id) {
            let files = ::semio_framework_async::poll::resolve_ready(probe_store.snapshot_pack()).map_err(|error| GatewayError::new(GatewayErrorCode::Internal, format!("snapshotting `{artifact_id}`: {error}")))?;
            return Ok(Some((files.pack, files.spr)));
        }
        #[cfg(not(target_arch = "wasm32"))]
        if let Some(binding) = self.plugin_artifact_binding(artifact_id) {
            if let Some(bytes) = self.read_session_artifact_bytes(&binding.plugin_id)? {
                return Ok(Some(bytes));
            }
            if let Some(bytes) = self.read_live_session_artifact_bytes(&binding.plugin_id, &binding.app_id)? {
                return Ok(Some(bytes));
            }
        }
        if matches!(self.origin, WorkspaceOrigin::Hub { .. }) {
            let snapshot = self.hub_snapshot()?;
            let Some(scope) = snapshot.documents.keys().find(|scope| scope.document_id == artifact_id).cloned() else {
                return Ok(None);
            };
            #[cfg(not(target_arch = "wasm32"))]
            {
                return self.read_hub_canonical_pair(&scope).map(Some);
            }
            #[cfg(target_arch = "wasm32")]
            {
                let _ = scope;
                return Err(GatewayError::new(GatewayErrorCode::PluginUnavailable, "native hub canonical pair transport is unavailable on wasm32").retryable());
            }
        }
        match &self.origin {
            WorkspaceOrigin::Folder { path } => {
                let storage = store::sync::FolderEventLogStorage::new(path.clone());
                ::semio_framework_async::poll::resolve_ready(storage.read(artifact_id)).map_err(|error| GatewayError::new(GatewayErrorCode::Internal, format!("reading `{artifact_id}`: {error}")))
            }
            WorkspaceOrigin::Hub { .. } => unreachable!("hub reads return before local probe or folder access"),
        }
    }

    /// 📖️ One hub document's canonical `pack`/`spr`, off the authenticated binding's own verified
    /// pair mount — the identical call `semio://workspace/scopes/{space}/{doc}/checkpoint` already
    /// makes, projected to the bytes. See [`remote::NativeHubBindingDriver::read_canonical_pair_bytes`].
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn read_hub_canonical_pair(&self, scope: &DocumentScope) -> Result<(Vec<u8>, Vec<u8>), GatewayError> {
        let binding = self.settled_hub_binding()?;
        let driver = self.hub_driver.as_ref().ok_or_else(|| GatewayError::new(GatewayErrorCode::PluginUnavailable, "hub binding actor is not running").retryable())?;
        let cancel = semio_framework_async::CancelToken::root_now();
        driver.read_canonical_pair_bytes(binding.as_ref(), scope, &cancel).map_err(remote::pair_mount_error_to_gateway)
    }

    /// ⏳️ The hub binding once any descriptor refresh in flight has settled (bounded by
    /// [`remote::HUB_AUTHORITY_SETTLE_WAIT_MS`]); the caller's own gate still fails closed.
    pub(crate) fn settled_hub_binding(&self) -> Result<&Arc<HubRemoteBinding>, GatewayError> {
        let binding = self.hub_binding.as_ref().ok_or_else(|| GatewayError::new(GatewayErrorCode::PluginUnavailable, "hub descriptor binding is unbound").retryable())?;
        binding.await_settled(remote::HUB_AUTHORITY_SETTLE_WAIT_MS);
        Ok(binding)
    }

    pub(crate) fn hub_snapshot(&self) -> Result<Arc<AuthorizedDescriptorSnapshot>, GatewayError> {
        let binding = self.settled_hub_binding()?;
        binding.ready_snapshot(i64::try_from(now_ms()).unwrap_or(i64::MAX))
    }

    /// 🗂️ The verified package roster and per-document dialects of the live descriptor authority.
    pub(crate) fn hub_catalog_snapshot(&self) -> Result<Arc<remote::AuthorizedCatalogSnapshot>, GatewayError> {
        let binding = self.settled_hub_binding()?;
        binding.ready_catalog_snapshot(i64::try_from(now_ms()).unwrap_or(i64::MAX))
    }
}
