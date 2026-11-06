//! 🗃️ Document load of the mounted artifact instance: the pure head hydration and the start of a document archive load. See [`super`].

use super::*;

impl<A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + Send + 'static> VcsArtifactApp<A, M> {
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
        let envelope = create_document_envelope::<A::Snapshot, A::Mutation>(A::DOCUMENT_SCHEMA, &id, head, None, &::semio_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
        let mut window_reset=Some(self.ladder_ref().prepare_document_window_reset()?);
        let actor = self.stores.store.local_actor_id().clone();
        self.stores.store.reset(loaded_with_app_dialect::<A>(envelope)).await.map_err(|error| error.into_fault())?;
        self.pending.commit_document_window_reset(&mut self.stores, &mut self.history, &mut window_reset, self.tools.tool_cancellations.scope_generation(), cx)?;
        assert_eq!(self.stores.store.local_actor_id(), &actor, "pure head retains its opened actor");
        self.history.cache = None;
        self.history.retire_displaced_document_rows();
        self.time_travel.set_history_unavailable(true);
        Ok(false)
    }

    pub(super) fn begin_document_archive_load(&mut self,operation:u64,archive:&mut protocol::DocumentArchivePack,admission:&mut semio_plugin_document_media::OriginalDocumentArchiveAdmission,merge:bool,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        if admission.completed(){admission.validate_step(operation,archive,cx)?;return if self.ingest.document_archive_loads.get(operation).is_some_and(|entry|entry.merge==merge){Ok(true)}else{Err(plugin_sdk_fault("completed archive admission lost its actual registry recipient").into())}}
        if !self.ingest.document_archive_loads.can_insert(operation){return Err(document_load_fault(DOCUMENT_LOAD_BUSY_CODE,"document archive operation identity is already live or its fixed slot is occupied").into())}
        if !admission.validate_step(operation,archive,cx)?{return Ok(false)}
        let capacity=std::mem::size_of::<protocol::DocumentArchivePack>();let bytes=std::mem::size_of::<ActiveDocumentArchiveLoad<A::Snapshot,A::Mutation>>()+capacity+std::mem::size_of::<u64>();
        let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items<2||grant.maximum_copy_bytes<bytes+std::mem::size_of_val(admission)||grant.maximum_capacity_bytes<capacity||grant.maximum_release_bytes<capacity||grant.maximum_depth<2||cx.fuel_remaining()<2{return Ok(false)}
        admission.transfer_step(cx,|cx|{
            let history=document_archive_history_decoder(archive.parent_spr.len()).map_err(|error|document_load_fault(DOCUMENT_LOAD_HISTORY_INVALID_CODE,error.to_string()))?;
            let mut active=ActiveDocumentArchiveLoad::new(operation,std::mem::take(archive),history);active.merge=merge;
            self.ingest.document_archive_loads.insert_admitted(operation,active);
            cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,retained_capacity_bytes:capacity,..Default::default()})?;cx.consume_fuel(1);Ok(())
        })
    }
}
