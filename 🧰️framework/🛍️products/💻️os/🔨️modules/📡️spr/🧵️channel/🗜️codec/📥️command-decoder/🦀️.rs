//! 📥️ Command decoder of the channel codec: the tag dispatch that decodes every AppCommand variant and the presence command sequence. See [`super`].

use super::*;

//#region 🔖️Codec
/// 📥️ Decodes one `AppCommand`, the inverse of [`encode_app_command`].
pub async fn decode_app_command(bytes: &[u8]) -> Result<AppCommand, protocol::ProtocolError> {
    let tag = *bytes.first().ok_or_else(|| malformed("channel app-command tag", 0, "empty frame"))?;
    let mut pos = 1usize;
    let command = match tag {
        0 => AppCommand::ConfigCommand { seq: protocol::read_varint_u64(bytes, &mut pos)?, command: protocol::read_bytes(bytes, &mut pos)? },
        1 => AppCommand::Command { seq: protocol::read_varint_u64(bytes, &mut pos)?, command: protocol::read_bytes(bytes, &mut pos)?, view_state: protocol::read_bytes(bytes, &mut pos)? },
        2 => AppCommand::CommandText { seq: protocol::read_varint_u64(bytes, &mut pos)?, line: protocol::read_str(bytes, &mut pos)? },
        3 => AppCommand::ContextMenu { seq: protocol::read_varint_u64(bytes, &mut pos)?, request: protocol::read_bytes(bytes, &mut pos)? },
        4 => AppCommand::ArtifactCommand { seq: protocol::read_varint_u64(bytes, &mut pos)?, command: protocol::read_bytes(bytes, &mut pos)? },
        5 => AppCommand::ApplyEnvelopes { seq: protocol::read_varint_u64(bytes, &mut pos)?, envelopes: read_vec_envelope(bytes, &mut pos).await? },
        7 => AppCommand::ReadDocument { seq: protocol::read_varint_u64(bytes, &mut pos)? },
        8 => AppCommand::LoadConfig { seq: protocol::read_varint_u64(bytes, &mut pos)?, pack: protocol::read_bytes(bytes, &mut pos)?, spr: protocol::read_bytes(bytes, &mut pos)? },
        9 => AppCommand::ReadConfig { seq: protocol::read_varint_u64(bytes, &mut pos)? },
        10 => AppCommand::MediaIn { seq: protocol::read_varint_u64(bytes, &mut pos)?, port: protocol::read_str(bytes, &mut pos)?, descriptor: protocol::read_bytes(bytes, &mut pos)?, data: protocol::read_bytes(bytes, &mut pos)? },
        11 => AppCommand::MediaOut { seq: protocol::read_varint_u64(bytes, &mut pos)?, port: protocol::read_str(bytes, &mut pos)?, request: protocol::read_bytes(bytes, &mut pos)? },
        12 => AppCommand::MediaFingerprint { seq: protocol::read_varint_u64(bytes, &mut pos)?, port: protocol::read_str(bytes, &mut pos)? },
        13 => AppCommand::PureCommand { seq: protocol::read_varint_u64(bytes, &mut pos)?, command: protocol::read_bytes(bytes, &mut pos)?, head: protocol::read_bytes(bytes, &mut pos)? },
        14 => AppCommand::LoadChildren { seq: protocol::read_varint_u64(bytes, &mut pos)?, entries: read_vec_child_pack(bytes, &mut pos).await? },
        15 => AppCommand::ReadChildren { seq: protocol::read_varint_u64(bytes, &mut pos)? },
        16 => AppCommand::ReadHistory { seq: protocol::read_varint_u64(bytes, &mut pos)? },
        17 => AppCommand::TransactionPrepare {
            seq: protocol::read_varint_u64(bytes, &mut pos)?,
            txn_id: protocol::read_str(bytes, &mut pos)?,
            mutation_id: protocol::read_str(bytes, &mut pos)?,
            payload: protocol::read_bytes(bytes, &mut pos)?,
            prepared_ops: read_vec_bytes(bytes, &mut pos).await?,
            origin: protocol::read_bytes(bytes, &mut pos)?,
            prepared_child_ops: protocol::read_bytes(bytes, &mut pos)?,
        },
        18 => AppCommand::TransactionCommit { seq: protocol::read_varint_u64(bytes, &mut pos)?, txn_id: protocol::read_str(bytes, &mut pos)? },
        19 => AppCommand::TransactionRollback { seq: protocol::read_varint_u64(bytes, &mut pos)?, txn_id: protocol::read_str(bytes, &mut pos)? },
        20 => AppCommand::TransactionUndo { seq: protocol::read_varint_u64(bytes, &mut pos)?, group_id: protocol::read_str(bytes, &mut pos)? },
        21 => AppCommand::TransactionRedo { seq: protocol::read_varint_u64(bytes, &mut pos)?, group_id: protocol::read_str(bytes, &mut pos)? },
        22 => {
            let seq = protocol::read_varint_u64(bytes, &mut pos)?;
            let artifact_ref = protocol::read_str(bytes, &mut pos)?;
            let role = *bytes.get(pos).ok_or_else(|| malformed("channel app-command OpenArtifact.role", pos as u64, "truncated"))?;
            pos += 1;
            let plugin_id = protocol::read_str(bytes, &mut pos)?;
            let app_id = protocol::read_str(bytes, &mut pos)?;
            AppCommand::OpenArtifact { seq, artifact_ref, role, plugin_id, app_id }
        }
        23 => {
            let seq = protocol::read_varint_u64(bytes, &mut pos)?;
            let artifact_kind = protocol::read_str(bytes, &mut pos)?;
            let standard = protocol::read_str(bytes, &mut pos)?;
            let subset = protocol::read_str(bytes, &mut pos)?;
            let role = *bytes.get(pos).ok_or_else(|| malformed("channel app-command SetDefaultApp.role", pos as u64, "truncated"))?;
            pos += 1;
            let plugin_id = protocol::read_str(bytes, &mut pos)?;
            let app_id = protocol::read_str(bytes, &mut pos)?;
            AppCommand::SetDefaultApp { seq, artifact_kind, standard, subset, role, plugin_id, app_id }
        }
        24 => {
            let seq = protocol::read_varint_u64(bytes, &mut pos)?;
            let artifact_kind = protocol::read_str(bytes, &mut pos)?;
            let standard = protocol::read_str(bytes, &mut pos)?;
            let subset = protocol::read_str(bytes, &mut pos)?;
            let role = *bytes.get(pos).ok_or_else(|| malformed("channel app-command ClearDefaultApp.role", pos as u64, "truncated"))?;
            AppCommand::ClearDefaultApp { seq, artifact_kind, standard, subset, role }
        }
        25 => {
            let seq = protocol::read_varint_u64(bytes, &mut pos)?;
            let policy = *bytes.get(pos).ok_or_else(|| malformed("channel app-command SetMergePolicy.policy", pos as u64, "truncated"))?;
            AppCommand::SetMergePolicy { seq, policy }
        }
        26 => {
            let seq = protocol::read_varint_u64(bytes, &mut pos)?;
            let conflict_id = protocol::read_str(bytes, &mut pos)?;
            let resolution = *bytes.get(pos).ok_or_else(|| malformed("channel app-command ResolveConflict.resolution", pos as u64, "truncated"))?;
            AppCommand::ResolveConflict { seq, conflict_id, resolution }
        }
        27 => AppCommand::ReadConflicts { seq: protocol::read_varint_u64(bytes, &mut pos)? },
        28 => return Err(malformed("channel presence command", pos as u64, "Presence requires reserve-before-decode PresenceCommandCursor admission")),
        29 => {
            let seq = protocol::read_varint_u64(bytes, &mut pos)?;
            let length = protocol::read_varint_u64(bytes, &mut pos)?;
            if length > 142 || length as usize != bytes.len().saturating_sub(pos) {
                return Err(malformed("local interaction command", pos as u64, "invalid exact bounded envelope"));
            }
            let command = protocol::decode_local_interaction_query_command(&bytes[pos..]).map_err(|reason| malformed("local interaction command", pos as u64, reason))?;
            AppCommand::LocalInteractionQuery { seq, command }
        }
        30 => AppCommand::LoadWindowConfig {
            seq: protocol::read_varint_u64(bytes, &mut pos)?,
            entry: WindowConfigPackEntry {
                window_id: protocol::read_str(bytes, &mut pos)?,
                window_kind_id: protocol::read_str(bytes, &mut pos)?,
                envelope_pack: protocol::read_bytes(bytes, &mut pos)?,
            },
        },
        31 => AppCommand::ReadWindowConfigs { seq: protocol::read_varint_u64(bytes, &mut pos)? },
        32 => AppCommand::LoadDocumentArchive { seq: protocol::read_varint_u64(bytes, &mut pos)?, archive: read_document_archive(bytes, &mut pos).await? },
        33 => AppCommand::ReadDocumentArchive { seq: protocol::read_varint_u64(bytes, &mut pos)? },
        34 => AppCommand::PollDocumentArchiveLoad { seq: protocol::read_varint_u64(bytes, &mut pos)?, operation: protocol::read_varint_u64(bytes, &mut pos)? },
        35 => AppCommand::CancelDocumentArchiveLoad { seq: protocol::read_varint_u64(bytes, &mut pos)?, operation: protocol::read_varint_u64(bytes, &mut pos)? },
        36 => AppCommand::AcknowledgeDocumentArchiveLoad { seq: protocol::read_varint_u64(bytes, &mut pos)?, operation: protocol::read_varint_u64(bytes, &mut pos)? },
        37 => AppCommand::SubmitMediaExport {
            seq: protocol::read_varint_u64(bytes, &mut pos)?,
            port: protocol::read_str(bytes, &mut pos)?,
            expected_parent_document_id: protocol::read_str(bytes, &mut pos)?,
            expected_base_revision: protocol::read_varint_u64(bytes, &mut pos)?,
            checkpoint_id: read_export_checkpoint(bytes,&mut pos)?,
        },
        38..=40 => {
            let seq = protocol::read_varint_u64(bytes, &mut pos)?;
            let handle = read_media_export_handle(bytes, &mut pos)?;
            match tag {
                38 => AppCommand::PollMediaExport { seq, handle },
                39 => AppCommand::CancelMediaExport { seq, handle },
                40 => AppCommand::TakeMediaExportChunk { seq, handle },
                _ => unreachable!(),
            }
        }
        41 => AppCommand::ReadDocumentIdentity { seq: protocol::read_varint_u64(bytes, &mut pos)? },
        42 => AppCommand::ReadChildHeads { seq: protocol::read_varint_u64(bytes, &mut pos)? },
        43 => AppCommand::MergeDocumentArchive { seq: protocol::read_varint_u64(bytes, &mut pos)?, archive: read_document_archive(bytes, &mut pos).await? },
        other => return Err(malformed("channel app-command tag", pos as u64, &format!("unknown tag {other:#x}"))),
    };
    Ok(command)
}

/// 🪪 Reads only the fixed Presence tag and sequence so the app can reserve its exact
/// item/byte owner before the roster decoder allocates or copies any entry.
pub fn presence_command_sequence(bytes: &[u8]) -> Result<Option<u64>, protocol::ProtocolError> {
    let Some(tag) = bytes.first().copied() else {
        return Err(malformed("channel app-command tag", 0, "empty frame"));
    };
    if tag != 28 {
        return Ok(None);
    }
    let mut pos = 1usize;
    Ok(Some(protocol::read_varint_u64(bytes, &mut pos)?))
}
//#endregion 🔖️Codec

fn read_export_checkpoint(bytes:&[u8],pos:&mut usize)->Result<Option<String>,protocol::ProtocolError>{let tag=*bytes.get(*pos).ok_or_else(||malformed("media export checkpoint",*pos as u64,"truncated selector"))?;*pos+=1;match tag{0=>Ok(None),1=>{let id=protocol::read_str(bytes,pos)?;if id.is_empty()||id.chars().count()>512{return Err(malformed("media export checkpoint",*pos as u64,"checkpoint identity exceeds its bound"))}Ok(Some(id))},_=>Err(malformed("media export checkpoint",*pos as u64,"selector must be zero or one"))}}
