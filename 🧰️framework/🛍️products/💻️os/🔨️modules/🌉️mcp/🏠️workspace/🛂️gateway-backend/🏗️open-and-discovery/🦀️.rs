//! 🏗️ Open and discovery of the headless workspace: construction, the folder and hub openers, the origin, catalog discovery, artifact byte reads and the settled hub snapshots. See [`super`].

use super::*;

impl HeadlessWorkspace {
    pub fn new(origin: WorkspaceOrigin, principal: String, scopes: Vec<String>, catalog: Arc<Catalog>, host_driver_policy:semio_framework_plugin_host::GuestRelayDriverPolicy, operation_policy:semio_framework_os_mcp_port::channel_port::HostOperationPolicy) -> Self {
        ensure_probe_codec_registered();
        let session_id = mint_session_id(&principal, 0);
        Self {
            #[cfg(any(test, feature = "testkit"))]
            test_discovery:None,
            host_driver_policy,
            operation_policy,
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

    pub fn open_folder(path: PathBuf, principal: String, scopes: Vec<String>, catalog: Arc<Catalog>, host_driver_policy:semio_framework_plugin_host::GuestRelayDriverPolicy, operation_policy:semio_framework_os_mcp_port::channel_port::HostOperationPolicy) -> Result<Self, GatewayError> {
        if !path.is_dir() {
            std::fs::create_dir_all(&path).map_err(|error| GatewayError::new(GatewayErrorCode::InputInvalid, format!("--folder `{}` does not exist and could not be created: {error}", path.display())))?;
        }
        Self::reject_repository_root_workspace(&path)?;
        Ok(Self::new(WorkspaceOrigin::Folder { path }, principal, scopes, catalog, host_driver_policy, operation_policy))
    }

    /// 🧩️ A hub-bound workspace runs the HUB's components, never a build tree that happens to
    /// sit on the same disk: the hub is the authority over what code opens its documents.
    pub fn open_hub(base_url: String, space_id: String, credential: Arc<LocalHubCredential>, principal: String, scopes: Vec<String>, host_driver_policy:semio_framework_plugin_host::GuestRelayDriverPolicy, operation_policy:semio_framework_os_mcp_port::channel_port::HostOperationPolicy) -> Result<Self, GatewayError> {
        remote::validate_hub_origin(&base_url, &space_id)?;
        #[cfg(not(target_arch = "wasm32"))]
        {
            let (binding, driver, grant_source) = NativeHubBindingDriver::connect(credential.clone(), &base_url, &space_id)?;
            binding.await_settled(remote::HUB_AUTHORITY_SETTLE_WAIT_MS);
            let catalog = WorkspaceCatalog::following(Arc::clone(&binding))?;
            let driver = Arc::new(driver);
            let mut workspace = Self::new(WorkspaceOrigin::Hub { base_url, space_id }, principal, scopes, catalog.current(), host_driver_policy, operation_policy);
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
        #[cfg(any(test, feature = "testkit"))]
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

    /// 🗿️ A hub document of another schema is not an ERROR, it is simply not a probe document. See [design notes](README.md).
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

    /// 🗂️ Every document id currently known to this workspace: the union of (a) every id this See [design notes](README.md).
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

    /// 📖️ One document's real pack+spr bytes, read from whoever OWNS that document right now: See [design notes](README.md).

    /// 📂️ Explicit folder reopen reads durable custody rather than a stale live alias.
    pub fn read_open_artifact_bytes(&self, artifact_id: &str) -> Result<Option<(Vec<u8>, Vec<u8>)>, GatewayError> {
        if let WorkspaceOrigin::Folder { path } = &self.origin {
            if !self.open_probes.lock().unwrap_or_else(std::sync::PoisonError::into_inner).contains_key(artifact_id) {
                return semio_framework_async::block_on(store::sync::FolderEventLogStorage::new(path.clone()).read(artifact_id)).map_err(|error| GatewayError::new(GatewayErrorCode::Internal, error.to_string()));
            }
        }
        self.read_artifact_bytes(artifact_id)
    }

    pub fn read_artifact_bytes(&self, artifact_id: &str) -> Result<Option<(Vec<u8>, Vec<u8>)>, GatewayError> {
        if let Some(probe_store) = self.open_probes.lock().unwrap_or_else(std::sync::PoisonError::into_inner).get(artifact_id) {
            let files = ::semio_framework_async::poll::resolve_ready(probe_store.snapshot_pack()).map_err(|error| GatewayError::new(GatewayErrorCode::Internal, format!("snapshotting `{artifact_id}`: {error}")))?;
            return Ok(Some((files.pack, files.spr)));
        }
        #[cfg(not(target_arch = "wasm32"))]
        if let Some(binding) = self.plugin_artifact_binding(artifact_id) {
            if let Some(bytes) = self.read_live_session_artifact_bytes(artifact_id, &binding.plugin_id, &binding.app_id)? {
                return Ok(Some(bytes));
            }
            if let Some(bytes) = self.read_session_artifact_bytes(&binding.plugin_id)? {
                return Ok(Some(bytes));
            }
            if let Some(pair) = binding.document { return Ok(Some((pair.pack.clone(), pair.spr.clone()))); }
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
