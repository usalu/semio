//! 🗃️ Document load of the mounted artifact instance: the pure head hydration and the start of a document archive load. See [`super`].

use super::*;

impl<A: ArtifactApp, M: SpaceMember + MemberFactory + Send + 'static> VcsArtifactApp<A, M> {
    /// 🫥️ A pure command's head is the HEAD snapshot without history (design §20.8, `📓️api-stepped-document-load.md` §4):
    /// it hydrates in O(snapshot) — no history fold — and marks the document head-only, so history verbs and queries answer
    /// `pure.history-unavailable`. An empty head keeps the live document.
    pub(super) async fn hydrate_pure_head(&mut self,head:&[u8],cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        if head.is_empty() {
            return Ok(true);
        }
        if self.pending.pending_document_window_admission.as_ref().is_some_and(|admission|admission.committed){return self.pending.close_document_window_admission(cx, false).map(|step|matches!(step,PluginLifecycleStep::Complete(_))).map_err(Into::into)}
        if !self.tools.admit_document_window_reset(&mut self.pending, cx)?{return Ok(false)}
        let head = <A::Snapshot as ArtifactPack>::decode_pack(head).map_err(|error| plugin_sdk_fault(format!("the pure head snapshot does not decode: {error:?}")))?;
        let id = self.stores.store.envelope().id.clone();
        let envelope = create_document_envelope::<A::Snapshot, A::Mutation>(A::DOCUMENT_SCHEMA, &id, head, None, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
        let mut window_reset=Some(self.ladder_ref().prepare_document_window_reset()?);
        let actor = self.stores.store.local_actor_id().clone();
        self.stores.store.reset(loaded_with_app_dialect::<A>(envelope)).await.map_err(|error| error.into_fault())?;
        self.pending.commit_document_window_reset(&mut self.stores, &mut self.history, &mut window_reset, cx)?;
        assert_eq!(self.stores.store.local_actor_id(), &actor, "pure head retains its opened actor");
        self.history.cache = None;
        self.history.retire_displaced_document_rows();
        self.time_travel.set_history_unavailable(true);
        Ok(false)
    }

    pub(super) fn begin_document_archive_load(&mut self, operation: u64, mut archive: protocol::DocumentArchivePack) -> Result<(), Fault> {
        if archive.members.len() > protocol::DOCUMENT_ARCHIVE_MAXIMUM_MEMBERS {
            return Err(document_load_fault(DOCUMENT_LOAD_TOO_MANY_MEMBERS_CODE, "document archive exceeds its fixed 1024-member authority").with_param("count", archive.members.len().to_string()).with_param("maximum", protocol::DOCUMENT_ARCHIVE_MAXIMUM_MEMBERS.to_string()));
        }
        let archive_payload_bytes = archive.members.iter().fold(archive.parent_pack.len().checked_add(archive.parent_spr.len()), |total, entry| total.and_then(|total| total.checked_add(entry.envelope_pack.len())));
        if archive_payload_bytes.is_none_or(|bytes| bytes > protocol::DOCUMENT_ARCHIVE_MAXIMUM_BYTES) {
            return Err(document_load_fault(DOCUMENT_LOAD_TOO_LARGE_CODE, "document archive typed payload exceeds its fixed byte authority").with_param("bytes", archive_payload_bytes.unwrap_or(usize::MAX).to_string()).with_param("maximum", protocol::DOCUMENT_ARCHIVE_MAXIMUM_BYTES.to_string()));
        }
        if !self.ingest.document_archive_loads.can_insert(operation) {
            return Err(document_load_fault(DOCUMENT_LOAD_BUSY_CODE, "document archive operation identity is already live or its fixed slot is occupied"));
        }
        if archive.parent_pack.is_empty() || archive.parent_spr.is_empty() {
            return Err(document_load_fault(DOCUMENT_LOAD_INCOMPLETE_CODE, "document archive parent pack and SPR must both be present"));
        }
        archive.members.sort_by_key(|entry| entry.ordinal);
        for (ordinal, entry) in archive.members.iter().enumerate() {
            let fields = [
                entry.reference.artifact_id.as_str(),
                entry.reference.artifact_kind.as_str(),
                entry.reference.standard.as_str(),
                entry.reference.subset.as_str(),
                entry.owner.parent.artifact_id.as_str(),
                entry.owner.parent.artifact_kind.as_str(),
                entry.owner.parent.standard.as_str(),
                entry.owner.parent.subset.as_str(),
                entry.owner.slot.as_str(),
                entry.owner.child_id.as_str(),
            ];
            if usize::try_from(entry.ordinal).ok() != Some(ordinal)
                || entry.envelope_pack.is_empty()
                || fields.iter().any(|field| field.is_empty() || field.len() > store::MEMBER_OPEN_IDENTITY_BYTES || field.chars().any(char::is_control))
            {
                return Err(document_load_fault(DOCUMENT_LOAD_MEMBER_INVALID_CODE, "document archive member ordinal, identity, owner, or envelope is invalid").with_param("ordinal", (ordinal + 1).to_string()));
            }
        }
        archive.members.reverse();
        let history = document_archive_history_decoder(archive.parent_spr.len()).map_err(|error| document_load_fault(DOCUMENT_LOAD_HISTORY_INVALID_CODE, error.to_string()))?;
        self.ingest.document_archive_loads.insert_admitted(operation, ActiveDocumentArchiveLoad::new(operation, archive, history));
        Ok(())
    }
}
