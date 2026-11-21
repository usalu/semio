//! 🔬️ Probe documents of the headless workspace: ensuring the probe artifact and its apply, undo and redo mutations. See [`super`].

use super::*;
use semio_os_mcp_probe_document::probe_document::{close_probe_store_to_terminal, probe_authoring_identity};
use store::os_vcs::io::binary::entity_identity::control::{EntityIdentityAuthority, Observer};
use store::ReferenceSpaceMember as _;

impl HeadlessWorkspace {
    /// 🎟️ Uses the workspace's declared transit and turn limits for one probe command.
    fn probe_identity<'a>(&self, observer: &'a mut Observer<'a>) -> Result<EntityIdentityAuthority<'a>, GatewayError> {
        let transit = self.operation_policy.transit;
        probe_authoring_identity(observer, transit.retained, transit.work_fuel, self.host_driver_policy.root.maximum_turn_microseconds)
            .map_err(|error| GatewayError::new(GatewayErrorCode::Internal, format!("admitting probe authoring: {error}")))
    }

    /// 🧪️ Opens the current probe or publishes its initial edit in the workspace.
    pub async fn ensure_probe_artifact(&self, artifact_id: &str, initial: semio_framework_value::DslValue) -> Result<RevisionStamp, GatewayError> {
        self.open_probe_artifact(artifact_id, initial, true).await
    }

    /// 🆕️ Publishes the initial edit only for a new probe artifact.
    pub async fn create_probe_artifact(&self, artifact_id: &str, initial: semio_framework_value::DslValue) -> Result<RevisionStamp, GatewayError> {
        self.open_probe_artifact(artifact_id, initial, false).await
    }

    async fn open_probe_artifact(&self, artifact_id: &str, initial: semio_framework_value::DslValue, allow_existing: bool) -> Result<RevisionStamp, GatewayError> {
        let already_exists = || GatewayError::new(GatewayErrorCode::PreconditionFailed, format!("artifact `{artifact_id}` already exists"));
        if let Some(probe) = self.open_probes.lock().unwrap_or_else(std::sync::PoisonError::into_inner).get(artifact_id) {
            return if allow_existing { Ok(probe_revision(artifact_id, probe)) } else { Err(already_exists()) };
        }
        let folder = self.probe_folder();
        let pair = match &folder {
            Some(storage) => storage.read(artifact_id).await.map_err(|error| probe_error(artifact_id, "reading", error))?,
            None => None,
        };
        if pair.is_some() && !allow_existing {
            return Err(already_exists());
        }
        let producer = ::semio_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer;
        let envelope = match &pair {
            Some((pack, spr)) => {
                if store::os_store::codec_registry::lane_schema_from_spr(spr).await.as_deref() != Some(PROBE_SCHEMA) {
                    return Err(GatewayError::new(GatewayErrorCode::PreconditionFailed, format!("artifact `{artifact_id}` has another kind")));
                }
                store::os_store::text_format::parse_document_pack::<ProbeSnapshot, ProbeMutation>(pack, spr, &producer)
                    .await.map_err(|error| probe_error(artifact_id, "opening", error))?.into_envelope()
            }
            None => store::os_store::document_materialize::create_document_envelope::<ProbeSnapshot, ProbeMutation>(PROBE_SCHEMA, artifact_id, ProbeSnapshot::default(), None, &producer),
        };
        let mut probe = ProbeStore::new::<store::os_store::ArtifactSnapshotPublication>(envelope, store::ActorId(self.actor_label().into()))
            .await.map_err(|error| probe_error(artifact_id, "constructing a store for", error))?;
        probe.install_document_store_owners_exact(probe_store_owners());

        let document_key = self.origin.artifact_document_key(artifact_id);
        if folder.is_none() {
            let channels = self.artifact_host.open(store::sync::ArtifactActorConfig {
                actor_identity_grant: semio_framework_value::RetainedCloneGrant { maximum_items: 1024, maximum_copy_bytes: 65536, maximum_capacity_bytes: 65536, maximum_release_bytes: 65536, maximum_depth: 64 },
                document_id: artifact_id.to_string(),
                schema: PROBE_SCHEMA.to_string(),
                bindings: vec![self.origin.persistence_binding()],
                watch_external: self.origin.watch_external(),
                actor: self.actor_label(),
            }).await;
            if channels.document_key != document_key {
                self.artifact_host.close_key(&channels.document_key);
                close_probe_store_to_terminal(probe);
                return Err(GatewayError::new(GatewayErrorCode::PluginUnavailable, format!("artifact `{artifact_id}` opened outside its authenticated document scope")).retryable());
            }
            if let Err(error) = probe.attach_backbone(store::os_store::backbone::Backbones::Channel(channels.channel_backbone)).await {
                self.artifact_host.close_key(&document_key);
                close_probe_store_to_terminal(probe);
                return Err(probe_error(artifact_id, "attaching a backbone for", error));
            }
        }
        if probe.applied_edit_ids().is_empty() && probe.redo_edit_ids().is_empty() {
            let command = ProbeCommand::Apply { mutations: vec![ProbeMutation::SetValue(initial)], transaction: None };
            let baseline = pair.as_ref().map(|(pack, spr)| (pack.as_slice(), spr.as_slice()));
            let seeded = self.commit_probe_command(artifact_id, &mut probe, command, folder.as_ref(), baseline).await;
            if let Err(error) = seeded {
                if folder.is_none() {
                    self.artifact_host.close_key(&document_key);
                }
                close_probe_store_to_terminal(probe);
                return Err(error);
            }
        }
        let revision = probe_revision(artifact_id, &probe);
        let displaced = self.open_probes.lock().unwrap_or_else(std::sync::PoisonError::into_inner).insert(artifact_id.to_string(), probe);
        if let Some(displaced) = displaced {
            close_probe_store_to_terminal(displaced);
        }
        Ok(revision)
    }

    /// 📁️ Resolves the folder event writer that commits an exact local probe journal.
    fn probe_folder(&self) -> Option<store::sync::FolderEventLogStorage> {
        match &self.origin {
            WorkspaceOrigin::Folder { path } => Some(store::sync::FolderEventLogStorage::new(path.clone())),
            WorkspaceOrigin::Hub { .. } => None,
        }
    }

    /// 💾️ Acknowledges a local command after its original owner pair reaches durable custody.
    async fn commit_probe_command(&self, artifact_id: &str, probe: &mut ProbeStore, command: ProbeCommand, folder: Option<&store::sync::FolderEventLogStorage>, baseline: Option<(&[u8], &[u8])>) -> Result<(), GatewayError> {
        let mut observe = |_| true;
        let mut identity = self.probe_identity(&mut observe)?;
        probe.dispatch(command, &mut identity).await.map_err(|error| probe_error(artifact_id, "editing", error))?;
        if let Some(storage) = folder {
            let files = probe.snapshot_pack().await.map_err(|error| probe_error(artifact_id, "capturing", error))?;
            let committed = match baseline {
                Some(baseline) => {
                    let archive = storage.read_archive(artifact_id).await.map_err(|error| probe_error(artifact_id, "reading the archive for", error))?;
                    storage.write_snapshot_if_current(artifact_id, PROBE_SCHEMA, baseline, archive.as_deref(), &files.pack, &files.spr).await.map(|event| event.is_some())
                }
                None => {
                    let archive = store::os_spr::encode_document_archive_bytes(&store::os_spr::DocumentArchivePack { parent_pack: files.pack, parent_spr: files.spr, members: Vec::new() })
                        .map_err(|error| probe_error(artifact_id, "encoding the archive for", error))?;
                    storage.create_archive_if_absent(artifact_id, PROBE_SCHEMA, &archive).await
                }
            }.map_err(|error| probe_error(artifact_id, "persisting", error))?;
            if !committed {
                return Err(GatewayError::new(GatewayErrorCode::PreconditionFailed, format!("artifact `{artifact_id}` changed in its folder")).retryable());
            }
        } else {
            self.artifact_host.send_key(&self.origin.artifact_document_key(artifact_id), store::sync::ArtifactActorMsg::LocalMutations { envelopes: Vec::new() }).await;
        }
        Ok(())
    }

    /// 🔁️ Loads the authoritative probe, commits a command and projects its result.
    async fn change_probe<T>(&self, artifact_id: &str, command: ProbeCommand, project: impl FnOnce(&ProbeStore) -> Result<T, GatewayError>) -> Result<T, GatewayError> {
        let folder = self.probe_folder();
        let mut probe = self.open_probes.lock().unwrap_or_else(std::sync::PoisonError::into_inner).remove(artifact_id)
            .ok_or_else(|| GatewayError::new(GatewayErrorCode::NotFound, format!("`{artifact_id}` has no open probe store")))?;
        let outcome = async {
            let before = match &folder {
                Some(_) => Some(probe.snapshot_pack().await.map_err(|error| probe_error(artifact_id, "capturing", error))?),
                None => None,
            };
            let baseline = before.as_ref().map(|files| (files.pack.as_slice(), files.spr.as_slice()));
            self.commit_probe_command(artifact_id, &mut probe, command, folder.as_ref(), baseline).await
        }.await;
        let result = outcome.and_then(|()| project(&probe));
        if result.is_err() && folder.is_some() {
            close_probe_store_to_terminal(probe);
        } else {
            let displaced = self.open_probes.lock().unwrap_or_else(std::sync::PoisonError::into_inner).insert(artifact_id.to_string(), probe);
            if let Some(displaced) = displaced {
                close_probe_store_to_terminal(displaced);
            }
        }
        result
    }

    /// ✍️ Commits the next probe value.
    pub async fn apply_probe_mutation(&self, artifact_id: &str, value: semio_framework_value::DslValue) -> Result<RevisionStamp, GatewayError> {
        self.ensure_probe_artifact(artifact_id, semio_framework_value::DslValue::Null).await?;
        self.change_probe(artifact_id, ProbeCommand::Apply { mutations: vec![ProbeMutation::SetValue(value)], transaction: None }, |probe| Ok(probe_revision(artifact_id, probe))).await
    }

    /// ↩️ Returns the probe value after the latest eligible edit is undone.
    pub async fn undo_probe_mutation(&self, artifact_id: &str) -> Result<semio_framework_value::DslValue, GatewayError> {
        self.change_probe(artifact_id, ProbeCommand::Undo, |probe| probe_value(artifact_id, probe)).await
    }

    /// ↪️ Returns the probe value after the latest eligible edit is redone.
    pub async fn redo_probe_mutation(&self, artifact_id: &str) -> Result<semio_framework_value::DslValue, GatewayError> {
        self.change_probe(artifact_id, ProbeCommand::Redo, |probe| probe_value(artifact_id, probe)).await
    }
}

type ProbeCommand = store::os_store::store_schema::ArtifactCommand<ProbeMutation>;

fn probe_revision(artifact_id: &str, probe: &ProbeStore) -> RevisionStamp {
    let ids = probe.applied_edit_ids();
    RevisionStamp { artifact_id: artifact_id.to_string(), head_edit_id: ids.last().cloned().unwrap_or_default(), cursor: ids.len().to_string() }
}

fn probe_value(artifact_id: &str, probe: &ProbeStore) -> Result<semio_framework_value::DslValue, GatewayError> {
    probe.snapshot().map(|snapshot| snapshot.0).map_err(|error| probe_error(artifact_id, "reading", error))
}

fn probe_error(artifact_id: &str, operation: &str, error: impl std::fmt::Display) -> GatewayError {
    GatewayError::new(GatewayErrorCode::Internal, format!("{operation} `{artifact_id}`: {error}"))
}
