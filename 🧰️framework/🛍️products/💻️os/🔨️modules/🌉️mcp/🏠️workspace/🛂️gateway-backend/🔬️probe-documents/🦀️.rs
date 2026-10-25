//! 🔬️ Probe documents of the headless workspace: ensuring the probe artifact and its apply, undo and redo mutations. See [`super`].

use super::*;
use store::ReferenceSpaceMember as _;

/// 🪪️ The authoring identity the headless probe documents issue to themselves for one command.
fn probe_identity() -> semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'static> {
    let observer: &'static mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'static> = Box::leak(Box::new(|_: semio_framework_value::native_encoding::NativeEncodeProgress| true));
    semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1usize << 24, observer).expect("headless probe authoring identity admission")
}

impl HeadlessWorkspace {
    /// 🧪️ Opens (creating on first use) a real `ProbeStore` bound through `ArtifactHost` — real VCS
    /// history, real backbone attachment (`ArtifactHost::send`/`subscribe` — §6 of the brief: "guest
    /// `SendMessage{Backbone}` effects route to `ArtifactHost::send`; inbound
    /// `ArtifactEvent::RemoteMutations` come back as guest events"). Applies `initial` as the FIRST
    /// commit only if the document has no history yet, so re-running this against an already-seeded
    /// workspace is idempotent and never double-commits. Returns the resulting `RevisionStamp`.
    pub async fn ensure_probe_artifact(&self, artifact_id: &str, initial: semio_framework_value::DslValue) -> Result<RevisionStamp, GatewayError> {
        let existing = self.open_probes.lock().unwrap_or_else(std::sync::PoisonError::into_inner).remove(artifact_id);
        let mut probe_store = match existing {
            Some(probe_store) => probe_store,
            None => {
                let envelope = store::os_store::document_materialize::create_document_envelope::<ProbeSnapshot, ProbeMutation>(PROBE_SCHEMA, artifact_id, ProbeSnapshot::default(), None, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
                let mut probe_store = ProbeStore::new::<store::os_store::ArtifactSnapshotPublication>(envelope, store::ActorId(self.actor_label().into())).await.map_err(|error| GatewayError::new(GatewayErrorCode::Internal, format!("constructing probe store for `{artifact_id}`: {error}")))?;
                probe_store.install_document_store_owners_exact(probe_store_owners());
                let document_key = self.origin.artifact_document_key(artifact_id);
                let channels = self
                    .artifact_host
                    .open(store::sync::ArtifactActorConfig { actor_identity_grant: semio_framework_value::RetainedCloneGrant {maximum_items:1024,maximum_copy_bytes:65536,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:64}, 
                        document_id: artifact_id.to_string(),
                        schema: PROBE_SCHEMA.to_string(),
                        bindings: vec![self.origin.persistence_binding()],
                        watch_external: self.origin.watch_external(),
                        actor: self.actor_label(),
                    })
                    .await;
                if channels.document_key != document_key {
                    self.artifact_host.close_key(&channels.document_key);
                    return Err(GatewayError::new(GatewayErrorCode::PluginUnavailable, format!("artifact `{artifact_id}` opened outside its authenticated document scope")).retryable());
                }
                probe_store.attach_backbone(store::os_store::backbone::Backbones::Channel(channels.channel_backbone)).await.map_err(|error| GatewayError::new(GatewayErrorCode::Internal, format!("attaching backbone for `{artifact_id}`: {error}")))?;
                probe_store
            }
        };
        if probe_store.applied_edit_ids().is_empty() {
            probe_store
                .dispatch(store::os_store::store_schema::ArtifactCommand::Apply { mutations: vec![ProbeMutation::SetValue(initial)], transaction: None }, &mut probe_identity())
                .await
                .map_err(|error| GatewayError::new(GatewayErrorCode::Internal, format!("seeding `{artifact_id}`: {error}")))?;
            self.artifact_host.send_key(&self.origin.artifact_document_key(artifact_id), store::sync::ArtifactActorMsg::LocalMutations { envelopes: Vec::new() }).await;
        }
        let applied_edit_ids = probe_store.applied_edit_ids();
        let head_edit_id = applied_edit_ids.last().cloned().unwrap_or_default();
        let revision = RevisionStamp { artifact_id: artifact_id.to_string(), head_edit_id, cursor: applied_edit_ids.len().to_string() };
        self.open_probes.lock().unwrap_or_else(std::sync::PoisonError::into_inner).insert(artifact_id.to_string(), probe_store);
        Ok(revision)
    }

    /// 🧪️ A real SECOND (or later) commit against an already-`ensure_probe_artifact`-opened
    /// document — proves genuine mutate-and-observe end to end for the ONE document kind this crate
    /// owns outright, independent of `ensure_probe_artifact`'s own idempotent-seed-only contract.
    /// This is deliberately scoped to `os.agent.probe/v1` — see this file's own module doc and
    /// `PluginArtifactChannel::exchange`'s `PureCommand`/`TransactionPrepare` arms for exactly why a
    /// generic, arbitrary-plugin equivalent is not real yet (a guest-dispatch gap in `🔌️plugin`'s
    /// own shared code, out of this packet's owned file).
    pub async fn apply_probe_mutation(&self, artifact_id: &str, value: semio_framework_value::DslValue) -> Result<RevisionStamp, GatewayError> {
        self.ensure_probe_artifact(artifact_id, semio_framework_value::DslValue::Null).await?;
        let mut probe_store =
            self.open_probes.lock().unwrap_or_else(std::sync::PoisonError::into_inner).remove(artifact_id).ok_or_else(|| GatewayError::new(GatewayErrorCode::Internal, format!("`{artifact_id}` was just ensured open but is missing from open_probes")))?;
        let dispatched = probe_store.dispatch(store::os_store::store_schema::ArtifactCommand::Apply { mutations: vec![ProbeMutation::SetValue(value)], transaction: None }, &mut probe_identity()).await;
        let applied_edit_ids = probe_store.applied_edit_ids();
        let head_edit_id = applied_edit_ids.last().cloned().unwrap_or_default();
        let cursor = applied_edit_ids.len().to_string();
        self.open_probes.lock().unwrap_or_else(std::sync::PoisonError::into_inner).insert(artifact_id.to_string(), probe_store);
        dispatched.map_err(|error| GatewayError::new(GatewayErrorCode::Internal, format!("applying mutation to `{artifact_id}`: {error}")))?;
        self.artifact_host.send_key(&self.origin.artifact_document_key(artifact_id), store::sync::ArtifactActorMsg::LocalMutations { envelopes: Vec::new() }).await;
        Ok(RevisionStamp { artifact_id: artifact_id.to_string(), head_edit_id, cursor })
    }

    /// 🧪️ Real `ArtifactStore::undo()` over an open probe document — same scope note as
    /// `apply_probe_mutation`.
    pub async fn undo_probe_mutation(&self, artifact_id: &str) -> Result<semio_framework_value::DslValue, GatewayError> {
        let mut probe_store =
            self.open_probes.lock().unwrap_or_else(std::sync::PoisonError::into_inner).remove(artifact_id).ok_or_else(|| GatewayError::new(GatewayErrorCode::NotFound, format!("`{artifact_id}` has no open probe store to undo")))?;
        let outcome = probe_store.undo(&mut probe_identity()).await;
        let snapshot = probe_store.snapshot();
        self.open_probes.lock().unwrap_or_else(std::sync::PoisonError::into_inner).insert(artifact_id.to_string(), probe_store);
        outcome.map_err(|error| GatewayError::new(GatewayErrorCode::Internal, format!("undoing `{artifact_id}`: {error}")))?;
        Ok(snapshot.map_err(|error| GatewayError::new(GatewayErrorCode::Internal, format!("reading `{artifact_id}` after undo: {error}")))?.0)
    }

    /// 🧪️ Real `ArtifactStore::redo()` over an open probe document — same scope note as
    /// `apply_probe_mutation`.
    pub async fn redo_probe_mutation(&self, artifact_id: &str) -> Result<semio_framework_value::DslValue, GatewayError> {
        let mut probe_store =
            self.open_probes.lock().unwrap_or_else(std::sync::PoisonError::into_inner).remove(artifact_id).ok_or_else(|| GatewayError::new(GatewayErrorCode::NotFound, format!("`{artifact_id}` has no open probe store to redo")))?;
        let outcome = probe_store.redo(&mut probe_identity()).await;
        let snapshot = probe_store.snapshot();
        self.open_probes.lock().unwrap_or_else(std::sync::PoisonError::into_inner).insert(artifact_id.to_string(), probe_store);
        outcome.map_err(|error| GatewayError::new(GatewayErrorCode::Internal, format!("redoing `{artifact_id}`: {error}")))?;
        Ok(snapshot.map_err(|error| GatewayError::new(GatewayErrorCode::Internal, format!("reading `{artifact_id}` after redo: {error}")))?.0)
    }
}
