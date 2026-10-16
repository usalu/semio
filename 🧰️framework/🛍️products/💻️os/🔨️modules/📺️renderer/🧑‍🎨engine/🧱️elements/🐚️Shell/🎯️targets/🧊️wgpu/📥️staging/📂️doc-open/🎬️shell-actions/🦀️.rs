//! 📂️ `shell-doc-open` staging of `shell-actions`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellActions
impl OpeningState {
    //#region 🔖️CheckIn
    /// 📌️ Drives the Check In by at most one bounded hub request per frame: waits (bounded) for the
    /// document's sync status to name an acknowledged head, submits it, then polls or cancels to a
    /// terminal status. Answers whether the footer's status changed.
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn pump_hub_check_in(&mut self, directory: &DirectoryState, sync: &mut SyncState) -> bool {
        use semio_framework_os_kernel::os_directory::{DocumentCheckInPhaseV1, DocumentCheckInRefusalV1, DocumentCheckInV1, DOCUMENT_CHECK_IN_SCHEMA_V1};
        let Some(operation) = sync.hub_check_in.clone() else { return false };
        if operation.status.as_ref().is_some_and(|status| status.phase.is_terminal()) {
            return false;
        }
        let now_ms = directory_now_ms();
        let local_refusal = |refusal| Some(hub_check_in_local_status(&operation.request_id, DocumentCheckInPhaseV1::Failed, Some(refusal)));
        let Some(client) = directory.directory_client.clone() else {
            sync.hub_check_in.as_mut().expect("armed check-in").status = local_refusal(DocumentCheckInRefusalV1::AuthorityChanged);
            return true;
        };
        if now_ms >= operation.deadline_at_ms {
            sync.hub_check_in.as_mut().expect("armed check-in").status = local_refusal(DocumentCheckInRefusalV1::Unavailable);
            return true;
        }
        let mounted = sync.sync_channel.as_ref().is_some_and(|channel| channel.document_id == operation.document_id) && self.open_space_id.as_deref() == Some(operation.space_id.as_str());
        let Some(head) = operation.head.clone() else {
            if !mounted || now_ms >= operation.quiescent_by_ms {
                sync.hub_check_in.as_mut().expect("armed check-in").status = local_refusal(DocumentCheckInRefusalV1::Unavailable);
                return true;
            }
            let Some(head) = sync.sync_status.as_ref().and_then(|status| status.acknowledged_head.clone()).filter(|head| head.document_id == operation.document_id) else { return false };
            let current = sync.hub_check_in.as_mut().expect("armed check-in");
            current.head = Some(head);
            current.status = Some(hub_check_in_local_status(&current.request_id, DocumentCheckInPhaseV1::Accepted, None));
            return true;
        };
        let cancel_due = operation.cancel_requested && !operation.cancel_sent;
        if operation.submitted && !cancel_due && now_ms < operation.next_poll_at_ms {
            return false;
        }
        let context = directory.directory_command_ctx();
        let answer = if !operation.submitted {
            let request = DocumentCheckInV1 { schema: DOCUMENT_CHECK_IN_SCHEMA_V1.to_string(), request_id: operation.request_id.clone(), head };
            client.document_check_in(&context, &operation.space_id, &request).await
        } else if cancel_due {
            client.cancel_document_check_in(&context, &operation.space_id, &operation.document_id, &operation.request_id).await
        } else {
            client.document_check_in_status(&context, &operation.space_id, &operation.document_id, &operation.request_id).await
        };
        let Some(current) = sync.hub_check_in.as_mut().filter(|current| current.request_id == operation.request_id) else { return false };
        current.next_poll_at_ms = directory_now_ms().saturating_add(HUB_CHECK_IN_POLL_MS);
        current.submitted = true;
        current.cancel_sent |= cancel_due;
        match answer {
            Ok(status) => current.status = Some(status),
            Err(DirectoryClientError::Unauthorized) | Err(DirectoryClientError::Http { status: 403, .. }) => {
                current.status = Some(hub_check_in_local_status(&current.request_id, DocumentCheckInPhaseV1::Failed, Some(DocumentCheckInRefusalV1::AuthorityChanged)))
            }
            Err(DirectoryClientError::Http { status, .. }) if (400..500).contains(&status) => current.status = Some(hub_check_in_local_status(&current.request_id, DocumentCheckInPhaseV1::Failed, Some(DocumentCheckInRefusalV1::Unavailable))),
            Err(_) => {}
        }
        true
    }

    //#endregion 🔖️CheckIn
    /// 🪪️ A hub document's execution-target lease for the exact scope and the surface this shell requests: its
    /// `checkpoint` is what the document's canonical pair must be ([`ShellHubDocumentSeed`]), and for a kind this
    /// process resolves no codec for (neither linked nor a mounted component's,
    /// [`store_sync::os_store::codec_registry::document_kind_codec`]) it is also the kind identity, bound onto the document host
    /// only when it names the package this shell mounted ([`document_execution_target_admitted`]). A hub
    /// resolution that already read the lease hands its `checkpoint` in, and a kind that resolves a codec then
    /// needs no second read. A document with no hub binding has no seed. Both targets walk this one body.
    #[allow(clippy::too_many_arguments)]
    pub async fn bind_document_execution_target(
        &mut self,
        document_id: &str,
        schema: &str,
        bindings: &[PersistenceBinding],
        program: &ProgramBridgeEntry,
        app: &AppDefinition,
        window_kind_id: &str,
        checkpoint: Option<semio_framework_os_kernel::os_directory::DocumentOpenCheckpointV1>, directory: &mut DirectoryState, sync: &SyncState
    ) -> Result<Option<ShellHubDocumentSeed>, String> {
        let Some(space_id) = bindings.iter().find_map(|binding| match binding {
            PersistenceBinding::Hub { space_id, .. } => Some(space_id.clone()),
            PersistenceBinding::Folder { .. } => None,
        }) else {
            return Ok(None);
        };
        let client = directory.directory_client.clone().ok_or("document open requires a signed-in hub session")?;
        let scope = semio_framework_os_kernel::os_directory::DocumentScope::new(space_id.as_str(), document_id);
        let codec_resolves = store_sync::os_store::codec_registry::document_kind_codec(schema).await.map_err(|error| format!("document codec registry: {error}"))?.is_some();
        let surface_id = wgpu_document_socket_surface(program, app, window_kind_id)?;
        let intent = semio_framework_os_kernel::os_directory::DocumentOpenIntentV1 {
            schema: "semio.hub.document-open-intent/v1".into(),
            version: 1,
            scope: semio_framework_os_kernel::os_directory::DocumentScope::new(space_id.as_str(), document_id),
            requested_surface_id: Some(surface_id.clone()),
            client_instance_id: format!("wgpu-shell-{}", directory.shell_session_id),
        };
        let lease = client.document_execution_target_manifest(&directory.directory_command_ctx(), &intent).await.map_err(|error| format!("document execution target: {error}"))?;
        let checkpoint = lease.checkpoint.clone();
        document_execution_target_admitted(&lease, &program.plugin_id, program.package_id.as_deref(), program.component_sha256.as_deref(), schema, &surface_id)?;
        #[cfg(not(target_arch="wasm32"))]
        directory.cancel_inference_port();
        self.document_execution_target_lease=Some(lease.clone());
        if !codec_resolves {
            document_execution_target_admitted(&lease, &program.plugin_id, program.package_id.as_deref(), program.component_sha256.as_deref(), schema, &surface_id)?;
            if !sync.document_host.set_document_execution_target_lease(&ArtifactDocumentKey::hub(space_id, document_id), lease) {
                return Err("document execution target is already bound for this document".into());
            }
        }
        Ok(Some(ShellHubDocumentSeed { client, ctx: directory.directory_ctx(), scope, checkpoint }))
    }

    /// 📇️ Default bindings for a document opened against the CURRENTLY mounted session's own
    /// dialect/role — `default_persistence_bindings`'s pure decision, fed the live identity/space/
    /// data-dir/surface this shell already holds.
    pub fn default_bindings_for_current_session(&self, directory: &DirectoryState, docs: &DocumentState) -> (Vec<PersistenceBinding>, Option<String>) {
        let space_id = self.open_space_id.clone();
        let data_dir = directory.identity_env.as_ref().and_then(|env| env.data_dir.as_deref());
        let surface = docs.session.as_ref().filter(|_| directory.identity.is_some() && space_id.is_some()).map(|session| semio_framework::manifest::surface_app_id(&session.app.dialect, session.app.role));
        let bindings = default_persistence_bindings(directory.identity.as_ref(), space_id.as_deref(), data_dir, surface.as_deref());
        (bindings, surface)
    }
}

//#endregion ShellActions

