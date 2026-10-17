//! 📤️ Command encoder of the channel codec: the tag dispatch that encodes every AppCommand variant. See [`super`].

use super::*;

//#region 🔖️Codec
/// 📄️ Produces the exact pre-admitted page owner consumed by reactor command batches.
pub async fn encode_app_command(command: &AppCommand) -> Result<PagedCommand, semio_framework_diagnostic::Fault> {
    if let AppCommand::Presence { own_color, peers, .. } = command {
        let mut pages = CommandPageSet::try_new(peers.len().max(1))?;
        for peer in peers.iter() {
            pages.try_push(FixedCommandPage::try_copy_from(peer)?).map_err(|(fault, _page)| fault)?;
        }
        if pages.is_empty() {
            pages.try_push(FixedCommandPage::try_copy_from(&[])?).map_err(|(fault, _page)| fault)?;
        }
        return PagedCommand::try_from_presence_pages(*own_color, pages, peers.len()).map_err(|(fault, _pages)| fault);
    }
    let mut out = CommandPageWriter::try_new()?;
    match command {
        AppCommand::ConfigCommand { seq, command } => {
            out.byte(0)?;
            out.varint(*seq)?;
            out.bytes(command)?;
        }
        AppCommand::Command { seq, command, view_state } => {
            out.byte(1)?;
            out.varint(*seq)?;
            out.bytes(command)?;
            out.bytes(view_state)?;
        }
        AppCommand::CommandText { seq, line } => {
            out.byte(2)?;
            out.varint(*seq)?;
            out.string(line)?;
        }
        AppCommand::ContextMenu { seq, request } => {
            out.byte(3)?;
            out.varint(*seq)?;
            out.bytes(request)?;
        }
        AppCommand::ArtifactCommand { seq, command } => {
            out.byte(4)?;
            out.varint(*seq)?;
            out.bytes(command)?;
        }
        AppCommand::ApplyEnvelopes { seq, envelopes } => {
            out.byte(5)?;
            out.varint(*seq)?;
            out.varint(envelopes.len() as u64)?;
            for envelope in envelopes {
                out.envelope(envelope)?;
            }
        }
        AppCommand::ReadDocument { seq } => {
            out.byte(7)?;
            out.varint(*seq)?;
        }
        AppCommand::LoadConfig { seq, pack, spr } => {
            out.byte(8)?;
            out.varint(*seq)?;
            out.bytes(pack)?;
            out.bytes(spr)?;
        }
        AppCommand::ReadConfig { seq } => {
            out.byte(9)?;
            out.varint(*seq)?;
        }
        AppCommand::LoadWindowConfig { seq, entry } => {
            out.byte(30)?;
            out.varint(*seq)?;
            out.string(&entry.window_id)?;
            out.string(&entry.window_kind_id)?;
            out.bytes(&entry.envelope_pack)?;
        }
        AppCommand::ReadWindowConfigs { seq } => {
            out.byte(31)?;
            out.varint(*seq)?;
        }
        AppCommand::MediaIn { seq, port, descriptor, data } => {
            out.byte(10)?;
            out.varint(*seq)?;
            out.string(port)?;
            out.bytes(descriptor)?;
            out.bytes(data)?;
        }
        AppCommand::MediaOut { seq, port, request } => {
            out.byte(11)?;
            out.varint(*seq)?;
            out.string(port)?;
            out.bytes(request)?;
        }
        AppCommand::MediaFingerprint { seq, port } => {
            out.byte(12)?;
            out.varint(*seq)?;
            out.string(port)?;
        }
        AppCommand::PureCommand { seq, command, head } => {
            out.byte(13)?;
            out.varint(*seq)?;
            out.bytes(command)?;
            out.bytes(head)?;
        }
        AppCommand::LoadChildren { seq, entries } => {
            out.byte(14)?;
            out.varint(*seq)?;
            out.varint(entries.len() as u64)?;
            for entry in entries {
                out.string(&entry.slot)?;
                out.string(&entry.child_id)?;
                out.string(&entry.dialect)?;
                out.bytes(&entry.envelope_pack)?;
                out.string(&entry.owner)?;
            }
        }
        AppCommand::ReadChildren { seq } => {
            out.byte(15)?;
            out.varint(*seq)?;
        }
        AppCommand::ReadHistory { seq } => {
            out.byte(16)?;
            out.varint(*seq)?;
        }
        AppCommand::TransactionPrepare { seq, txn_id, mutation_id, payload, prepared_ops, origin, prepared_child_ops } => {
            if prepared_ops.len() > TRANSACTION_PREPARED_OPS_MAXIMUM {
                return Err(semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.transaction-prepared-ops"), "transaction prepare exceeds its fixed 1024 prepared-op authority"));
            }
            out.byte(17)?;
            out.varint(*seq)?;
            out.string(txn_id)?;
            out.string(mutation_id)?;
            out.bytes(payload)?;
            out.varint(prepared_ops.len() as u64)?;
            for op in prepared_ops {
                out.bytes(op)?;
            }
            out.bytes(origin)?;
            out.bytes(prepared_child_ops)?;
        }
        AppCommand::TransactionCommit { seq, txn_id } => {
            out.byte(18)?;
            out.varint(*seq)?;
            out.string(txn_id)?;
        }
        AppCommand::TransactionRollback { seq, txn_id } => {
            out.byte(19)?;
            out.varint(*seq)?;
            out.string(txn_id)?;
        }
        AppCommand::TransactionUndo { seq, group_id } => {
            out.byte(20)?;
            out.varint(*seq)?;
            out.string(group_id)?;
        }
        AppCommand::TransactionRedo { seq, group_id } => {
            out.byte(21)?;
            out.varint(*seq)?;
            out.string(group_id)?;
        }
        AppCommand::OpenArtifact { seq, artifact_ref, role, plugin_id, app_id } => {
            out.byte(22)?;
            out.varint(*seq)?;
            out.string(artifact_ref)?;
            out.byte(*role)?;
            out.string(plugin_id)?;
            out.string(app_id)?;
        }
        AppCommand::SetDefaultApp { seq, artifact_kind, standard, subset, role, plugin_id, app_id } => {
            out.byte(23)?;
            out.varint(*seq)?;
            out.string(artifact_kind)?;
            out.string(standard)?;
            out.string(subset)?;
            out.byte(*role)?;
            out.string(plugin_id)?;
            out.string(app_id)?;
        }
        AppCommand::ClearDefaultApp { seq, artifact_kind, standard, subset, role } => {
            out.byte(24)?;
            out.varint(*seq)?;
            out.string(artifact_kind)?;
            out.string(standard)?;
            out.string(subset)?;
            out.byte(*role)?;
        }
        AppCommand::SetMergePolicy { seq, policy } => {
            out.byte(25)?;
            out.varint(*seq)?;
            out.byte(*policy)?;
        }
        AppCommand::ResolveConflict { seq, conflict_id, resolution } => {
            out.byte(26)?;
            out.varint(*seq)?;
            out.string(conflict_id)?;
            out.byte(*resolution)?;
        }
        AppCommand::ReadConflicts { seq } => {
            out.byte(27)?;
            out.varint(*seq)?;
        }
        AppCommand::LocalInteractionQuery { seq, command } => {
            out.byte(29)?;
            out.varint(*seq)?;
            out.bytes(&protocol::encode_local_interaction_query_command(command))?;
        }
        AppCommand::LoadDocumentArchive { seq, archive } | AppCommand::MergeDocumentArchive { seq, archive } => {
            if archive.members.len() > DOCUMENT_ARCHIVE_MAXIMUM_MEMBERS {
                return Err(semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.document-archive-members"), "document archive exceeds its fixed 1024-member authority"));
            }
            out.byte(if matches!(command, AppCommand::MergeDocumentArchive { .. }) { 43 } else { 32 })?;
            out.varint(*seq)?;
            out.bytes(&archive.parent_pack)?;
            out.bytes(&archive.parent_spr)?;
            out.varint(archive.members.len() as u64)?;
            for entry in &archive.members {
                out.varint(u64::from(entry.ordinal))?;
                out.string(&entry.reference.artifact_id)?;
                out.string(&entry.reference.artifact_kind)?;
                out.string(&entry.reference.standard)?;
                out.string(&entry.reference.subset)?;
                out.string(&entry.owner.parent.artifact_id)?;
                out.string(&entry.owner.parent.artifact_kind)?;
                out.string(&entry.owner.parent.standard)?;
                out.string(&entry.owner.parent.subset)?;
                out.string(&entry.owner.slot)?;
                out.string(&entry.owner.child_id)?;
                out.bytes(&entry.envelope_pack)?;
            }
        }
        AppCommand::ReadDocumentArchive { seq } => {
            out.byte(33)?;
            out.varint(*seq)?;
        }
        AppCommand::PollDocumentArchiveLoad { seq, operation } => {
            out.byte(34)?;
            out.varint(*seq)?;
            out.varint(*operation)?;
        }
        AppCommand::CancelDocumentArchiveLoad { seq, operation } => {
            out.byte(35)?;
            out.varint(*seq)?;
            out.varint(*operation)?;
        }
        AppCommand::AcknowledgeDocumentArchiveLoad { seq, operation } => {
            out.byte(36)?;
            out.varint(*seq)?;
            out.varint(*operation)?;
        }
        AppCommand::SubmitMediaExport { seq, port, expected_parent_document_id, expected_base_revision } => {
            out.byte(37)?;
            out.varint(*seq)?;
            out.string(port)?;
            out.string(expected_parent_document_id)?;
            out.varint(*expected_base_revision)?;
        }
        AppCommand::PollMediaExport { seq, handle } | AppCommand::CancelMediaExport { seq, handle } | AppCommand::TakeMediaExportChunk { seq, handle } => {
            let tag = match command {
                AppCommand::PollMediaExport { .. } => 38,
                AppCommand::CancelMediaExport { .. } => 39,
                AppCommand::TakeMediaExportChunk { .. } => 40,
                _ => unreachable!(),
            };
            out.byte(tag)?;
            out.varint(*seq)?;
            write_media_export_handle_paged(&mut out, handle)?;
        }
        AppCommand::ReadDocumentIdentity { seq } => {
            out.byte(41)?;
            out.varint(*seq)?;
        }
        AppCommand::ReadChildHeads { seq } => {
            out.byte(42)?;
            out.varint(*seq)?;
        }
        AppCommand::Presence { .. } => unreachable!(),
    }
    out.finish()
}
//#endregion 🔖️Codec
