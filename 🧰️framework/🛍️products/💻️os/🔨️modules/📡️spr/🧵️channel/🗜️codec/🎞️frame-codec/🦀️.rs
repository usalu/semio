//! 🎞️ Frame codec of the channel codec: the AppFrame encoder and decoder and the local interaction query frame writer. See [`super`].

use super::*;

//#region 🔖️Codec
/// 📤️ Encodes one `AppFrame`: `tag u8 | fields`.
pub async fn encode_app_frame(frame: &AppFrame) -> Vec<u8> {
    let mut out = Vec::new();
    match frame {
        AppFrame::LocalInteractionQuery { reply } => {
            out.reserve_exact(4260);
            encode_local_interaction_query_frame_into(reply, &mut out).expect("typed local query frame fits admitted wire extent");
        }
        AppFrame::Done { in_reply_to } => {
            out.push(0);
            protocol::write_varint_u64(&mut out, *in_reply_to);
        }
        AppFrame::Invocation { in_reply_to, output, diagnostics, ui_scope, history_patch, messages, mutations, inverse_group } => {
            assert!(mutations.len() <= INVOCATION_RESULT_PACK_MAXIMUM_BYTES && inverse_group.len() <= INVOCATION_RESULT_PACK_MAXIMUM_BYTES, "invocation result pack exceeds command transport authority");
            out.push(1);
            protocol::write_varint_u64(&mut out, *in_reply_to);
            protocol::write_bytes(&mut out, output);
            protocol::write_bytes(&mut out, diagnostics);
            protocol::write_bytes(&mut out, ui_scope);
            protocol::write_bytes(&mut out, history_patch);
            protocol::write_bytes(&mut out, messages);
            protocol::write_bytes(&mut out, mutations);
            protocol::write_bytes(&mut out, inverse_group);
        }
        AppFrame::DocumentChanged { envelopes, origin } => {
            out.push(2);
            write_vec_envelope(&mut out, envelopes).await;
            protocol::write_str(&mut out, origin);
        }
        AppFrame::Document { in_reply_to, pack, spr, ops } => {
            out.push(3);
            protocol::write_varint_u64(&mut out, *in_reply_to);
            protocol::write_bytes(&mut out, pack);
            protocol::write_bytes(&mut out, spr);
            protocol::write_str(&mut out, ops);
        }
        AppFrame::Config { in_reply_to, pack, spr, ops } => {
            out.push(4);
            protocol::write_varint_u64(&mut out, *in_reply_to);
            protocol::write_bytes(&mut out, pack);
            protocol::write_bytes(&mut out, spr);
            protocol::write_str(&mut out, ops);
        }
        AppFrame::WindowConfigs { in_reply_to, entries } => {
            out.push(24);
            protocol::write_varint_u64(&mut out, *in_reply_to);
            write_vec_window_config_pack(&mut out, entries).await;
        }
        AppFrame::ConfigChanged { envelopes, origin } => {
            out.push(5);
            write_vec_envelope(&mut out, envelopes).await;
            protocol::write_str(&mut out, origin);
        }
        AppFrame::ContextMenu { in_reply_to, items } => {
            out.push(6);
            protocol::write_varint_u64(&mut out, *in_reply_to);
            protocol::write_bytes(&mut out, items);
        }
        AppFrame::Media { in_reply_to, port, descriptor, data } => {
            out.push(7);
            protocol::write_varint_u64(&mut out, *in_reply_to);
            protocol::write_str(&mut out, port);
            protocol::write_bytes(&mut out, descriptor);
            protocol::write_bytes(&mut out, data);
        }
        AppFrame::MediaFingerprint { in_reply_to, port, fingerprint } => {
            out.push(8);
            protocol::write_varint_u64(&mut out, *in_reply_to);
            protocol::write_str(&mut out, port);
            protocol::write_bytes(&mut out, fingerprint);
        }
        AppFrame::Error { in_reply_to, fault, report } => {
            out.push(9);
            write_opt_u64(&mut out, in_reply_to).await;
            protocol::write_bytes(&mut out, fault);
            protocol::write_bytes(&mut out, report);
        }
        AppFrame::Emit { in_reply_to, document_ops, config_ops, draft_ops, output, diagnostics, child_ops } => {
            out.push(10);
            protocol::write_varint_u64(&mut out, *in_reply_to);
            protocol::write_bytes(&mut out, document_ops);
            protocol::write_bytes(&mut out, config_ops);
            protocol::write_bytes(&mut out, draft_ops);
            protocol::write_bytes(&mut out, output);
            protocol::write_bytes(&mut out, diagnostics);
            protocol::write_bytes(&mut out, child_ops);
        }
        AppFrame::Draft { in_reply_to, pack, spr, ops } => {
            out.push(11);
            protocol::write_varint_u64(&mut out, *in_reply_to);
            protocol::write_bytes(&mut out, pack);
            protocol::write_bytes(&mut out, spr);
            protocol::write_str(&mut out, ops);
        }
        AppFrame::Children { in_reply_to, entries } => {
            out.push(12);
            protocol::write_varint_u64(&mut out, *in_reply_to);
            write_vec_child_pack(&mut out, entries).await;
        }
        AppFrame::Ephemeral { presence, presence_generation, transient_generation, interaction, tool_run, history_edit } => {
            out.push(13);
            protocol::write_bytes(&mut out, presence);
            protocol::write_varint_u64(&mut out, *presence_generation);
            protocol::write_varint_u64(&mut out, *transient_generation);
            protocol::write_bytes(&mut out, interaction);
            protocol::write_bytes(&mut out, tool_run);
            protocol::write_bytes(&mut out, history_edit);
        }
        AppFrame::HistorySnapshot { in_reply_to, history_patch } => {
            out.push(14);
            protocol::write_varint_u64(&mut out, *in_reply_to);
            protocol::write_bytes(&mut out, history_patch);
        }
        AppFrame::TransactionProposal { in_reply_to, proposal_id, local_ops, foreign } => {
            out.push(15);
            protocol::write_varint_u64(&mut out, *in_reply_to);
            protocol::write_str(&mut out, proposal_id);
            write_vec_bytes(&mut out, local_ops).await;
            write_vec_bytes(&mut out, foreign).await;
        }
        AppFrame::TransactionPrepared { txn_id, foreign, rejection } => {
            out.push(16);
            protocol::write_str(&mut out, txn_id);
            write_vec_bytes(&mut out, foreign).await;
            protocol::write_bytes(&mut out, rejection);
        }
        AppFrame::TransactionCommitted { txn_id, edit_id } => {
            out.push(17);
            protocol::write_str(&mut out, txn_id);
            protocol::write_str(&mut out, edit_id);
        }
        AppFrame::TransactionRolledBack { txn_id } => {
            out.push(18);
            protocol::write_str(&mut out, txn_id);
        }
        AppFrame::MergeReport { in_reply_to, report } => {
            out.push(19);
            write_opt_u64(&mut out, in_reply_to).await;
            protocol::write_bytes(&mut out, report);
        }
        AppFrame::Conflicts { in_reply_to, conflicts } => {
            out.push(20);
            write_opt_u64(&mut out, in_reply_to).await;
            protocol::write_bytes(&mut out, conflicts);
        }
        AppFrame::UiPatch { in_reply_to, surface, kind, revision, base_revision, ops } => {
            out.push(21);
            write_opt_u64(&mut out, in_reply_to).await;
            protocol::write_str(&mut out, surface);
            protocol::write_str(&mut out, kind);
            protocol::write_varint_u64(&mut out, *revision);
            protocol::write_varint_u64(&mut out, *base_revision);
            protocol::write_bytes(&mut out, ops);
        }
        AppFrame::UiSnapshotEnd { revision } => {
            out.push(22);
            protocol::write_varint_u64(&mut out, *revision);
        }
        AppFrame::OperationCompleted { operation, revision, ui_scope, history_patch } => {
            out.push(25);
            protocol::write_varint_u64(&mut out, *operation);
            protocol::write_varint_u64(&mut out, *revision);
            protocol::write_bytes(&mut out, ui_scope);
            protocol::write_bytes(&mut out, history_patch);
        }
        AppFrame::DocumentArchive { in_reply_to, archive } => {
            assert!(archive.members.len() <= DOCUMENT_ARCHIVE_MAXIMUM_MEMBERS, "document archive exceeds its fixed 1024-member authority");
            out.push(26);
            protocol::write_varint_u64(&mut out, *in_reply_to);
            write_document_archive(&mut out, archive);
        }
        AppFrame::DocumentArchiveLoad { in_reply_to, status } => {
            out.push(27);
            protocol::write_varint_u64(&mut out, *in_reply_to);
            protocol::write_varint_u64(&mut out, status.operation);
            out.push(status.state.wire());
            protocol::write_varint_u64(&mut out, status.completed);
            protocol::write_varint_u64(&mut out, status.total);
            protocol::write_varint_u64(&mut out, status.ahead);
            protocol::write_bytes(&mut out, &status.fault);
        }
        AppFrame::MediaExportSubmitted { in_reply_to, handle } => {
            out.push(28);
            protocol::write_varint_u64(&mut out, *in_reply_to);
            write_media_export_handle(&mut out, handle);
        }
        AppFrame::MediaExportStatus { in_reply_to, handle, state, applied_progress, checkpoint_available, mime_type, total_bytes, detail } => {
            out.push(29);
            protocol::write_varint_u64(&mut out, *in_reply_to);
            write_media_export_handle(&mut out, handle);
            out.push(state.wire());
            protocol::write_varint_u64(&mut out, *applied_progress);
            protocol::write_bool(&mut out, *checkpoint_available);
            protocol::write_str(&mut out, mime_type);
            protocol::write_varint_u64(&mut out, *total_bytes);
            protocol::write_str(&mut out, detail);
        }
        AppFrame::MediaExportChunk { in_reply_to, handle, data, terminal } => {
            out.push(30);
            protocol::write_varint_u64(&mut out, *in_reply_to);
            write_media_export_handle(&mut out, handle);
            protocol::write_bytes(&mut out, data);
            protocol::write_bool(&mut out, *terminal);
        }
        AppFrame::DocumentIdentity { in_reply_to, identity } => {
            out.push(31);
            protocol::write_varint_u64(&mut out, *in_reply_to);
            protocol::write_varint_u64(&mut out, u64::from(identity.app_instance_id));
            protocol::write_bool(&mut out, identity.parent_document_id.is_some());
            if let Some(id) = &identity.parent_document_id {
                protocol::write_str(&mut out, id);
            }
            protocol::write_varint_u64(&mut out, identity.base_revision);
        }
        AppFrame::ChildHeads { in_reply_to, entries } => {
            out.push(32);
            protocol::write_varint_u64(&mut out, *in_reply_to);
            write_vec_child_head(&mut out, entries);
        }
    }
    out
}

/// 📥️ Decodes one `AppFrame`, the inverse of [`encode_app_frame`].
pub async fn decode_app_frame(bytes: &[u8]) -> Result<AppFrame, protocol::ProtocolError> {
    let tag = *bytes.first().ok_or_else(|| malformed("channel app-frame tag", 0, "empty frame"))?;
    let mut pos = 1usize;
    let frame = match tag {
        0 => AppFrame::Done { in_reply_to: protocol::read_varint_u64(bytes, &mut pos)? },
        1 => AppFrame::Invocation {
            in_reply_to: protocol::read_varint_u64(bytes, &mut pos)?,
            output: protocol::read_bytes(bytes, &mut pos)?,
            diagnostics: protocol::read_bytes(bytes, &mut pos)?,
            ui_scope: protocol::read_bytes(bytes, &mut pos)?,
            history_patch: protocol::read_bytes(bytes, &mut pos)?,
            messages: protocol::read_bytes(bytes, &mut pos)?,
            mutations: read_invocation_result_pack(bytes, &mut pos)?,
            inverse_group: read_invocation_result_pack(bytes, &mut pos)?,
        },
        2 => AppFrame::DocumentChanged { envelopes: read_vec_envelope(bytes, &mut pos).await?, origin: protocol::read_str(bytes, &mut pos)? },
        3 => AppFrame::Document { in_reply_to: protocol::read_varint_u64(bytes, &mut pos)?, pack: protocol::read_bytes(bytes, &mut pos)?, spr: protocol::read_bytes(bytes, &mut pos)?, ops: protocol::read_str(bytes, &mut pos)? },
        4 => AppFrame::Config { in_reply_to: protocol::read_varint_u64(bytes, &mut pos)?, pack: protocol::read_bytes(bytes, &mut pos)?, spr: protocol::read_bytes(bytes, &mut pos)?, ops: protocol::read_str(bytes, &mut pos)? },
        5 => AppFrame::ConfigChanged { envelopes: read_vec_envelope(bytes, &mut pos).await?, origin: protocol::read_str(bytes, &mut pos)? },
        6 => AppFrame::ContextMenu { in_reply_to: protocol::read_varint_u64(bytes, &mut pos)?, items: protocol::read_bytes(bytes, &mut pos)? },
        7 => {
            AppFrame::Media { in_reply_to: protocol::read_varint_u64(bytes, &mut pos)?, port: protocol::read_str(bytes, &mut pos)?, descriptor: protocol::read_bytes(bytes, &mut pos)?, data: protocol::read_bytes(bytes, &mut pos)? }
        }
        8 => AppFrame::MediaFingerprint { in_reply_to: protocol::read_varint_u64(bytes, &mut pos)?, port: protocol::read_str(bytes, &mut pos)?, fingerprint: protocol::read_bytes(bytes, &mut pos)? },
        9 => AppFrame::Error { in_reply_to: read_opt_u64(bytes, &mut pos).await?, fault: protocol::read_bytes(bytes, &mut pos)?, report: protocol::read_bytes(bytes, &mut pos)? },
        10 => AppFrame::Emit {
            in_reply_to: protocol::read_varint_u64(bytes, &mut pos)?,
            document_ops: protocol::read_bytes(bytes, &mut pos)?,
            config_ops: protocol::read_bytes(bytes, &mut pos)?,
            draft_ops: protocol::read_bytes(bytes, &mut pos)?,
            output: protocol::read_bytes(bytes, &mut pos)?,
            diagnostics: protocol::read_bytes(bytes, &mut pos)?,
            child_ops: protocol::read_bytes(bytes, &mut pos)?,
        },
        11 => AppFrame::Draft { in_reply_to: protocol::read_varint_u64(bytes, &mut pos)?, pack: protocol::read_bytes(bytes, &mut pos)?, spr: protocol::read_bytes(bytes, &mut pos)?, ops: protocol::read_str(bytes, &mut pos)? },
        12 => AppFrame::Children { in_reply_to: protocol::read_varint_u64(bytes, &mut pos)?, entries: read_vec_child_pack(bytes, &mut pos).await? },
        13 => AppFrame::Ephemeral {
            presence: protocol::read_bytes(bytes, &mut pos)?,
            presence_generation: protocol::read_varint_u64(bytes, &mut pos)?,
            transient_generation: protocol::read_varint_u64(bytes, &mut pos)?,
            interaction: protocol::read_bytes(bytes, &mut pos)?,
            tool_run: protocol::read_bytes(bytes, &mut pos)?,
            history_edit: protocol::read_bytes(bytes, &mut pos)?,
        },
        14 => AppFrame::HistorySnapshot { in_reply_to: protocol::read_varint_u64(bytes, &mut pos)?, history_patch: protocol::read_bytes(bytes, &mut pos)? },
        15 => AppFrame::TransactionProposal {
            in_reply_to: protocol::read_varint_u64(bytes, &mut pos)?,
            proposal_id: protocol::read_str(bytes, &mut pos)?,
            local_ops: read_vec_bytes(bytes, &mut pos).await?,
            foreign: read_vec_bytes(bytes, &mut pos).await?,
        },
        16 => AppFrame::TransactionPrepared { txn_id: protocol::read_str(bytes, &mut pos)?, foreign: read_vec_bytes(bytes, &mut pos).await?, rejection: protocol::read_bytes(bytes, &mut pos)? },
        17 => AppFrame::TransactionCommitted { txn_id: protocol::read_str(bytes, &mut pos)?, edit_id: protocol::read_str(bytes, &mut pos)? },
        18 => AppFrame::TransactionRolledBack { txn_id: protocol::read_str(bytes, &mut pos)? },
        19 => AppFrame::MergeReport { in_reply_to: read_opt_u64(bytes, &mut pos).await?, report: protocol::read_bytes(bytes, &mut pos)? },
        20 => AppFrame::Conflicts { in_reply_to: read_opt_u64(bytes, &mut pos).await?, conflicts: protocol::read_bytes(bytes, &mut pos)? },
        21 => {
            let in_reply_to = read_opt_u64(bytes, &mut pos).await?;
            let surface = protocol::read_str(bytes, &mut pos)?;
            let kind = protocol::read_str(bytes, &mut pos)?;
            let revision = protocol::read_varint_u64(bytes, &mut pos)?;
            let base_revision = protocol::read_varint_u64(bytes, &mut pos)?;
            let ops = protocol::read_bytes(bytes, &mut pos)?;
            AppFrame::UiPatch { in_reply_to, surface, kind, revision, base_revision, ops }
        }
        22 => AppFrame::UiSnapshotEnd { revision: protocol::read_varint_u64(bytes, &mut pos)? },
        23 => {
            let length = protocol::read_varint_u64(bytes, &mut pos)?;
            if length > 4256 || length as usize != bytes.len().saturating_sub(pos) {
                return Err(malformed("local interaction reply", pos as u64, "invalid exact bounded envelope"));
            }
            let reply = protocol::decode_local_interaction_query_reply(&bytes[pos..]).map_err(|reason| malformed("local interaction reply", pos as u64, reason))?;
            pos = bytes.len();
            AppFrame::LocalInteractionQuery { reply }
        }
        24 => AppFrame::WindowConfigs {
            in_reply_to: protocol::read_varint_u64(bytes, &mut pos)?,
            entries: read_vec_window_config_pack(bytes, &mut pos).await?,
        },
        25 => AppFrame::OperationCompleted {
            operation: protocol::read_varint_u64(bytes, &mut pos)?,
            revision: protocol::read_varint_u64(bytes, &mut pos)?,
            ui_scope: protocol::read_bytes(bytes, &mut pos)?,
            history_patch: protocol::read_bytes(bytes, &mut pos)?,
        },
        26 => AppFrame::DocumentArchive { in_reply_to: protocol::read_varint_u64(bytes, &mut pos)?, archive: read_document_archive(bytes, &mut pos).await? },
        27 => {
            let in_reply_to = protocol::read_varint_u64(bytes, &mut pos)?;
            let operation = protocol::read_varint_u64(bytes, &mut pos)?;
            let state = DocumentArchiveLoadState::from_wire(*bytes.get(pos).ok_or_else(|| malformed("document archive load state", pos as u64, "truncated"))?)?;
            pos += 1;
            let completed = protocol::read_varint_u64(bytes, &mut pos)?;
            let total = protocol::read_varint_u64(bytes, &mut pos)?;
            let ahead = protocol::read_varint_u64(bytes, &mut pos)?;
            let fault = protocol::read_bytes(bytes, &mut pos)?;
            AppFrame::DocumentArchiveLoad { in_reply_to, status: DocumentArchiveLoadStatus { operation, state, completed, total, ahead, fault } }
        }
        28 => AppFrame::MediaExportSubmitted {
            in_reply_to: protocol::read_varint_u64(bytes, &mut pos)?,
            handle: read_media_export_handle(bytes, &mut pos)?,
        },
        29 => {
            let in_reply_to = protocol::read_varint_u64(bytes, &mut pos)?;
            let handle = read_media_export_handle(bytes, &mut pos)?;
            let state = MediaExportStateWire::from_wire(*bytes.get(pos).ok_or_else(|| malformed("media export state", pos as u64, "truncated"))?)?;
            pos += 1;
            let applied_progress = protocol::read_varint_u64(bytes, &mut pos)?;
            let checkpoint_available = protocol::read_bool(bytes, &mut pos)?;
            let mime_type = protocol::read_str(bytes, &mut pos)?;
            let total_bytes = protocol::read_varint_u64(bytes, &mut pos)?;
            let detail = protocol::read_str(bytes, &mut pos)?;
            AppFrame::MediaExportStatus { in_reply_to, handle, state, applied_progress, checkpoint_available, mime_type, total_bytes, detail }
        }
        30 => AppFrame::MediaExportChunk {
            in_reply_to: protocol::read_varint_u64(bytes, &mut pos)?,
            handle: read_media_export_handle(bytes, &mut pos)?,
            data: protocol::read_bytes(bytes, &mut pos)?,
            terminal: protocol::read_bool(bytes, &mut pos)?,
        },
        31 => {
            if bytes.len() > 2_077 {
                return Err(malformed("channel document identity", pos as u64, "identity frame byte budget exceeded"));
            }
            let in_reply_to = protocol::read_varint_u64(bytes, &mut pos)?;
            let app_instance_id = u32::try_from(protocol::read_varint_u64(bytes, &mut pos)?).map_err(|_| malformed("channel document identity", pos as u64, "app instance exceeds u32"))?;
            let presence = *bytes.get(pos).ok_or_else(|| malformed("channel document identity", pos as u64, "truncated document presence"))?;
            pos += 1;
            if presence > 1 {
                return Err(malformed("channel document identity", pos as u64, "document presence must be zero or one"));
            }
            let parent_document_id = if presence == 1 {
                let id = protocol::read_str(bytes, &mut pos)?;
                if !(1..=512).contains(&id.chars().count()) {
                    return Err(malformed("channel document identity", pos as u64, "document id length refused"));
                }
                Some(id)
            } else { None };
            AppFrame::DocumentIdentity { in_reply_to, identity: AppDocumentIdentity { app_instance_id, parent_document_id, base_revision: protocol::read_varint_u64(bytes, &mut pos)? } }
        }
        32 => AppFrame::ChildHeads { in_reply_to: protocol::read_varint_u64(bytes, &mut pos)?, entries: read_vec_child_head(bytes, &mut pos)? },
        other => return Err(malformed("channel app-frame tag", pos as u64, &format!("unknown tag {other:#x}"))),
    };
    if pos != bytes.len() {
        return Err(malformed("channel app-frame", pos as u64, "trailing bytes after the frame"));
    }
    Ok(frame)
}
//#endregion 🔖️Codec

/// 📤️ Atomic bounded frame encoding into caller-admitted storage; failure never modifies output.
pub fn encode_local_interaction_query_frame_into(reply: &protocol::LocalInteractionQueryReply, out: &mut Vec<u8>) -> Result<(), &'static str> {
    let length = protocol::local_interaction_query_reply_encoded_len(reply)?;
    let prefix = 1 + ((usize::BITS - length.leading_zeros()).max(1) as usize).div_ceil(7);
    if out.capacity() - out.len() < prefix + length {
        return Err("local-interaction.frame-not-admitted");
    }
    out.push(23);
    protocol::write_varint_u64(out, length as u64);
    protocol::encode_local_interaction_query_reply_into(reply, out)
}
