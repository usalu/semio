//! 📚️ Archive and pack codecs of the channel codec: the child pack and head, document archive and window config pack readers and writers. See [`super`].

use super::*;

//#region 🔖️Codec
/// 🧸️ `count varint | (slot, child_id, dialect, envelope_pack, owner)*` — the shared list codec for
/// both `AppCommand::LoadChildren` and `AppFrame::Children`.
pub(super) async fn write_vec_child_pack(out: &mut Vec<u8>, entries: &[ChildPackEntry]) {
    protocol::write_varint_u64(out, entries.len() as u64);
    for entry in entries {
        protocol::write_str(out, &entry.slot);
        protocol::write_str(out, &entry.child_id);
        protocol::write_str(out, &entry.dialect);
        protocol::write_bytes(out, &entry.envelope_pack);
        protocol::write_str(out, &entry.owner);
    }
}

/// 🪆️ `count varint | (slot, child_id, dialect, head_pack, owner)*` — `AppFrame::ChildHeads`' list codec.
pub(super) fn write_vec_child_head(out: &mut Vec<u8>, entries: &[ChildHeadPackEntry]) {
    protocol::write_varint_u64(out, entries.len() as u64);
    for entry in entries {
        protocol::write_str(out, &entry.slot);
        protocol::write_str(out, &entry.child_id);
        protocol::write_str(out, &entry.dialect);
        protocol::write_bytes(out, &entry.head_pack);
        protocol::write_str(out, &entry.owner);
    }
}

/// 🪆️ Inverse of [`write_vec_child_head`]; the declared count never reserves past the archive's member authority.
pub(super) fn read_vec_child_head(bytes: &[u8], pos: &mut usize) -> Result<Vec<ChildHeadPackEntry>, protocol::ProtocolError> {
    let count = protocol::read_varint_u64(bytes, pos)?;
    if count > DOCUMENT_ARCHIVE_MAXIMUM_MEMBERS as u64 {
        return Err(malformed("channel child heads", *pos as u64, "child head count exceeds the member authority"));
    }
    let mut entries = Vec::with_capacity(count as usize);
    for _ in 0..count {
        entries.push(ChildHeadPackEntry { slot: protocol::read_str(bytes, pos)?, child_id: protocol::read_str(bytes, pos)?, dialect: protocol::read_str(bytes, pos)?, head_pack: protocol::read_bytes(bytes, pos)?, owner: protocol::read_str(bytes, pos)? });
    }
    Ok(entries)
}

/// 🧸️ Inverse of [`write_vec_child_pack`].
pub(super) async fn read_vec_child_pack(bytes: &[u8], pos: &mut usize) -> Result<Vec<ChildPackEntry>, protocol::ProtocolError> {
    let count = protocol::read_varint_u64(bytes, pos)?;
    let mut entries = Vec::with_capacity(count as usize);
    for _ in 0..count {
        entries.push(ChildPackEntry { slot: protocol::read_str(bytes, pos)?, child_id: protocol::read_str(bytes, pos)?, dialect: protocol::read_str(bytes, pos)?, envelope_pack: protocol::read_bytes(bytes, pos)?, owner: protocol::read_str(bytes, pos)? });
    }
    Ok(entries)
}

pub(super) fn write_document_archive(out: &mut Vec<u8>, archive: &DocumentArchivePack) {
    protocol::write_bytes(out, &archive.parent_pack);
    protocol::write_bytes(out, &archive.parent_spr);
    protocol::write_varint_u64(out, archive.members.len() as u64);
    for entry in &archive.members {
        protocol::write_varint_u64(out, u64::from(entry.ordinal));
        for label in archive_member_labels(entry) {
            protocol::write_str(out, label);
        }
        protocol::write_bytes(out, &entry.envelope_pack);
    }
}

fn archive_member_labels(entry: &OwnedDocumentMemberPackEntry) -> [&str; 10] {
    [
        &entry.reference.artifact_id, &entry.reference.artifact_kind,
        &entry.reference.standard, &entry.reference.subset,
        &entry.owner.parent.artifact_id, &entry.owner.parent.artifact_kind,
        &entry.owner.parent.standard, &entry.owner.parent.subset,
        &entry.owner.slot, &entry.owner.child_id,
    ]
}

fn archive_wire_size(archive: &DocumentArchivePack) -> usize {
    let varint = |value: u64| ((u64::BITS - value.leading_zeros()).div_ceil(7) as usize).max(1);
    let field = |bytes: &[u8]| varint(bytes.len() as u64).saturating_add(bytes.len());
    let header = 1 + varint(archive.members.len() as u64);
    let mut size = header.saturating_add(field(&archive.parent_pack)).saturating_add(field(&archive.parent_spr));
    for entry in &archive.members {
        size = size.saturating_add(varint(u64::from(entry.ordinal)));
        for label in archive_member_labels(entry) {
            size = size.saturating_add(field(label.as_bytes()));
        }
        size = size.saturating_add(field(&entry.envelope_pack));
    }
    size
}

/// 🗃️ Encodes one exact persisted recursive-document archive with an explicit version owner.
pub fn encode_document_archive_bytes(archive: &DocumentArchivePack) -> Result<Vec<u8>, protocol::ProtocolError> {
    if archive.members.len() > DOCUMENT_ARCHIVE_MAXIMUM_MEMBERS {
        return Err(malformed("document archive member count", 0, "member count exceeds 1024"));
    }
    let size = archive_wire_size(archive);
    if size > DOCUMENT_ARCHIVE_MAXIMUM_BYTES {
        return Err(malformed("document archive bytes", 0, "archive exceeds its fixed byte authority"));
    }
    let mut bytes = Vec::new();
    bytes.try_reserve_exact(size).map_err(|_| malformed("document archive bytes", 0, "archive backing allocation refused"))?;
    bytes.push(DOCUMENT_ARCHIVE_VERSION);
    write_document_archive(&mut bytes, archive);
    debug_assert_eq!(bytes.len(), size);
    Ok(bytes)
}

pub(in super::super) fn document_archive_into_fields(mut archive: DocumentArchivePack) -> Vec<Vec<u8>> {
    let mut fields = Vec::with_capacity(archive.members.len().saturating_mul(11).saturating_add(2));
    fields.push(std::mem::take(&mut archive.parent_pack));
    fields.push(std::mem::take(&mut archive.parent_spr));
    for mut entry in archive.members.drain(..) {
        fields.push(std::mem::take(&mut entry.reference.artifact_id).into_bytes());
        fields.push(std::mem::take(&mut entry.reference.artifact_kind).into_bytes());
        fields.push(std::mem::take(&mut entry.reference.standard).into_bytes());
        fields.push(std::mem::take(&mut entry.reference.subset).into_bytes());
        fields.push(std::mem::take(&mut entry.owner.parent.artifact_id).into_bytes());
        fields.push(std::mem::take(&mut entry.owner.parent.artifact_kind).into_bytes());
        fields.push(std::mem::take(&mut entry.owner.parent.standard).into_bytes());
        fields.push(std::mem::take(&mut entry.owner.parent.subset).into_bytes());
        fields.push(std::mem::take(&mut entry.owner.slot).into_bytes());
        fields.push(std::mem::take(&mut entry.owner.child_id).into_bytes());
        fields.push(std::mem::take(&mut entry.envelope_pack));
    }
    fields
}

pub(super) async fn read_document_archive(bytes: &[u8], pos: &mut usize) -> Result<DocumentArchivePack, protocol::ProtocolError> {
    let parent_pack = protocol::read_bytes(bytes, pos)?;
    let parent_spr = protocol::read_bytes(bytes, pos)?;
    let count = usize::try_from(protocol::read_varint_u64(bytes, pos)?).map_err(|_| malformed("document archive member count", *pos as u64, "member count is not representable"))?;
    if count > DOCUMENT_ARCHIVE_MAXIMUM_MEMBERS {
        return Err(malformed("document archive member count", *pos as u64, "member count exceeds 1024"));
    }
    let mut members = Vec::new();
    members.try_reserve_exact(count).map_err(|_| malformed("document archive member count", *pos as u64, "member roster allocation failed"))?;
    for _ in 0..count {
        let ordinal = u32::try_from(protocol::read_varint_u64(bytes, pos)?).map_err(|_| malformed("document archive member ordinal", *pos as u64, "ordinal exceeds u32"))?;
        let reference = DocumentArchiveArtifactRef {
            artifact_id: protocol::read_str(bytes, pos)?,
            artifact_kind: protocol::read_str(bytes, pos)?,
            standard: protocol::read_str(bytes, pos)?,
            subset: protocol::read_str(bytes, pos)?,
        };
        let parent = DocumentArchiveArtifactRef {
            artifact_id: protocol::read_str(bytes, pos)?,
            artifact_kind: protocol::read_str(bytes, pos)?,
            standard: protocol::read_str(bytes, pos)?,
            subset: protocol::read_str(bytes, pos)?,
        };
        let owner = DocumentArchiveOwnerRef { parent, slot: protocol::read_str(bytes, pos)?, child_id: protocol::read_str(bytes, pos)? };
        let envelope_pack = protocol::read_bytes(bytes, pos)?;
        members.push(OwnedDocumentMemberPackEntry { ordinal, reference, owner, envelope_pack });
    }
    Ok(DocumentArchivePack { parent_pack, parent_spr, members })
}

/// 🗃️ Decodes one exact persisted recursive-document archive and rejects trailing ownership.
pub async fn decode_document_archive_bytes(bytes: &[u8]) -> Result<DocumentArchivePack, protocol::ProtocolError> {
    if bytes.is_empty() || bytes.len() > DOCUMENT_ARCHIVE_MAXIMUM_BYTES {
        return Err(malformed("document archive bytes", 0, "archive exceeds its fixed byte authority"));
    }
    if bytes[0] != DOCUMENT_ARCHIVE_VERSION {
        return Err(malformed("document archive version", 0, "unsupported or missing version"));
    }
    let mut position = 1;
    let archive = read_document_archive(bytes, &mut position).await?;
    if position != bytes.len() {
        return Err(malformed("document archive bytes", position as u64, "trailing bytes"));
    }
    Ok(archive)
}

/// 🪟️ Writes the exact concrete-window config pack roster.
pub(super) async fn write_vec_window_config_pack(out: &mut Vec<u8>, entries: &[WindowConfigPackEntry]) {
    protocol::write_varint_u64(out, entries.len() as u64);
    for entry in entries {
        protocol::write_str(out, &entry.window_id);
        protocol::write_str(out, &entry.window_kind_id);
        protocol::write_bytes(out, &entry.envelope_pack);
    }
}

/// 🪟️ Reads the exact concrete-window config pack roster.
pub(super) async fn read_vec_window_config_pack(bytes: &[u8], pos: &mut usize) -> Result<Vec<WindowConfigPackEntry>, protocol::ProtocolError> {
    let count = protocol::read_varint_u64(bytes, pos)?;
    let mut entries = Vec::with_capacity(count as usize);
    for _ in 0..count {
        entries.push(WindowConfigPackEntry {
            window_id: protocol::read_str(bytes, pos)?,
            window_kind_id: protocol::read_str(bytes, pos)?,
            envelope_pack: protocol::read_bytes(bytes, pos)?,
        });
    }
    Ok(entries)
}
//#endregion 🔖️Codec
