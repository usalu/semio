//! 📦️ Archive decode of the paged command decoders: the field limits, the document archive decode phases and members and the paged document archive decode. See [`super`].

use super::*;

//#region 🔖️PagedAppCommandDecode
pub(super) const APP_COMMAND_FIELD_MAXIMUM_BYTES: usize = COMMAND_MAXIMUM_BYTES;

/// 🧮️ `AppCommand::PureCommand`'s two byte fields, in `encode_app_command`'s own order: `command`, then the `head`
/// snapshot pack. One field per retained decode step, so a mutation's payload never costs more than one bounded read
/// per host turn.
pub(super) const PURE_COMMAND_FIELDS: usize = 2;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum DocumentArchiveDecodePhase {
    ParentPack,
    ParentSpr,
    MemberCount,
    Ordinal,
    ReferenceArtifactId,
    ReferenceKind,
    ReferenceStandard,
    ReferenceSubset,
    OwnerParentArtifactId,
    OwnerParentKind,
    OwnerParentStandard,
    OwnerParentSubset,
    OwnerSlot,
    OwnerChildId,
    EnvelopePack,
    Complete,
}

#[derive(Debug, Default)]
pub(super) struct PartialDocumentArchiveMember {
    pub(in super::super) ordinal: u32,
    pub(in super::super) reference: DocumentArchiveArtifactRef,
    pub(in super::super) owner: DocumentArchiveOwnerRef,
    pub(in super::super) envelope_pack: Vec<u8>,
}

#[derive(Debug)]
pub(super) struct PagedDocumentArchiveDecode {
    pub(in super::super) archive: DocumentArchivePack,
    pub(super) partial: PartialDocumentArchiveMember,
    pub(in super::super) remaining: usize,
    pub(super) phase: DocumentArchiveDecodePhase,
    pub(super) rejected: Option<Vec<u8>>,
    pub(super) closing_member: Option<OwnedDocumentMemberPackEntry>,
    pub(super) close_field: u8,
}

impl PagedDocumentArchiveDecode {
    pub(in super::super) fn new() -> Self {
        Self {
            archive: DocumentArchivePack::default(),
            partial: PartialDocumentArchiveMember::default(),
            remaining: 0,
            phase: DocumentArchiveDecodePhase::ParentPack,
            rejected: None,
            closing_member: None,
            close_field: 0,
        }
    }

    pub(super) fn closing(archive: DocumentArchivePack) -> Self {
        Self {
            archive,
            partial: PartialDocumentArchiveMember::default(),
            remaining: 0,
            phase: DocumentArchiveDecodePhase::Complete,
            rejected: None,
            closing_member: None,
            close_field: 0,
        }
    }

    pub(super) fn read_string(&mut self, reader: &mut PagedCommandReader, label: &'static str) -> Result<String, semio_framework_diagnostic::Fault> {
        let bytes = reader.read_bounded_bytes(APP_COMMAND_FIELD_MAXIMUM_BYTES)?;
        String::from_utf8(bytes).map_err(|error| {
            self.rejected = Some(error.into_bytes());
            semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.document-archive-utf8"), label)
        })
    }

    pub(in super::super) fn step(&mut self, reader: &mut PagedCommandReader) -> Result<Option<DocumentArchivePack>, semio_framework_diagnostic::Fault> {
        if self.rejected.is_some() {
            return Err(semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.document-archive-closing"), "rejected document archive must be closed before another decode step"));
        }
        match self.phase {
            DocumentArchiveDecodePhase::ParentPack => {
                self.archive.parent_pack = reader.read_bounded_bytes(APP_COMMAND_FIELD_MAXIMUM_BYTES)?;
                self.phase = DocumentArchiveDecodePhase::ParentSpr;
            }
            DocumentArchiveDecodePhase::ParentSpr => {
                self.archive.parent_spr = reader.read_bounded_bytes(APP_COMMAND_FIELD_MAXIMUM_BYTES)?;
                self.phase = DocumentArchiveDecodePhase::MemberCount;
            }
            DocumentArchiveDecodePhase::MemberCount => {
                let count = usize::try_from(reader.read_varint()?).map_err(|_| semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.document-archive-members"), "document archive member count is not representable"))?;
                if count > DOCUMENT_ARCHIVE_MAXIMUM_MEMBERS {
                    return Err(semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.document-archive-members"), "document archive exceeds its fixed 1024-member authority"));
                }
                self.archive.members.try_reserve_exact(count).map_err(|_| semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.document-archive-allocation"), "document archive member roster could not reserve its exact bounded authority"))?;
                self.remaining = count;
                self.phase = if count == 0 { DocumentArchiveDecodePhase::Complete } else { DocumentArchiveDecodePhase::Ordinal };
            }
            DocumentArchiveDecodePhase::Ordinal => {
                self.partial.ordinal = u32::try_from(reader.read_varint()?).map_err(|_| semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.document-archive-ordinal"), "document archive member ordinal exceeds u32"))?;
                self.phase = DocumentArchiveDecodePhase::ReferenceArtifactId;
            }
            DocumentArchiveDecodePhase::ReferenceArtifactId => {
                self.partial.reference.artifact_id = self.read_string(reader, "document archive reference artifact id is not UTF-8")?;
                self.phase = DocumentArchiveDecodePhase::ReferenceKind;
            }
            DocumentArchiveDecodePhase::ReferenceKind => {
                self.partial.reference.artifact_kind = self.read_string(reader, "document archive reference kind is not UTF-8")?;
                self.phase = DocumentArchiveDecodePhase::ReferenceStandard;
            }
            DocumentArchiveDecodePhase::ReferenceStandard => {
                self.partial.reference.standard = self.read_string(reader, "document archive reference standard is not UTF-8")?;
                self.phase = DocumentArchiveDecodePhase::ReferenceSubset;
            }
            DocumentArchiveDecodePhase::ReferenceSubset => {
                self.partial.reference.subset = self.read_string(reader, "document archive reference subset is not UTF-8")?;
                self.phase = DocumentArchiveDecodePhase::OwnerParentArtifactId;
            }
            DocumentArchiveDecodePhase::OwnerParentArtifactId => {
                self.partial.owner.parent.artifact_id = self.read_string(reader, "document archive owner parent artifact id is not UTF-8")?;
                self.phase = DocumentArchiveDecodePhase::OwnerParentKind;
            }
            DocumentArchiveDecodePhase::OwnerParentKind => {
                self.partial.owner.parent.artifact_kind = self.read_string(reader, "document archive owner parent kind is not UTF-8")?;
                self.phase = DocumentArchiveDecodePhase::OwnerParentStandard;
            }
            DocumentArchiveDecodePhase::OwnerParentStandard => {
                self.partial.owner.parent.standard = self.read_string(reader, "document archive owner parent standard is not UTF-8")?;
                self.phase = DocumentArchiveDecodePhase::OwnerParentSubset;
            }
            DocumentArchiveDecodePhase::OwnerParentSubset => {
                self.partial.owner.parent.subset = self.read_string(reader, "document archive owner parent subset is not UTF-8")?;
                self.phase = DocumentArchiveDecodePhase::OwnerSlot;
            }
            DocumentArchiveDecodePhase::OwnerSlot => {
                self.partial.owner.slot = self.read_string(reader, "document archive owner slot is not UTF-8")?;
                self.phase = DocumentArchiveDecodePhase::OwnerChildId;
            }
            DocumentArchiveDecodePhase::OwnerChildId => {
                self.partial.owner.child_id = self.read_string(reader, "document archive owner child id is not UTF-8")?;
                self.phase = DocumentArchiveDecodePhase::EnvelopePack;
            }
            DocumentArchiveDecodePhase::EnvelopePack => {
                self.partial.envelope_pack = reader.read_bounded_bytes(APP_COMMAND_FIELD_MAXIMUM_BYTES)?;
                let partial = std::mem::take(&mut self.partial);
                self.archive.members.push(OwnedDocumentMemberPackEntry { ordinal: partial.ordinal, reference: partial.reference, owner: partial.owner, envelope_pack: partial.envelope_pack });
                self.remaining -= 1;
                self.phase = if self.remaining == 0 { DocumentArchiveDecodePhase::Complete } else { DocumentArchiveDecodePhase::Ordinal };
            }
            DocumentArchiveDecodePhase::Complete => return Ok(Some(std::mem::take(&mut self.archive))),
        }
        if self.phase == DocumentArchiveDecodePhase::Complete {
            Ok(Some(std::mem::take(&mut self.archive)))
        } else {
            Ok(None)
        }
    }

    pub(super) fn close_string(value: &mut String, maximum_bytes: usize) -> Option<usize> {
        if value.is_empty() {
            return None;
        }
        let bytes = value.len();
        if bytes > maximum_bytes {
            return Some(0);
        }
        let value = std::mem::take(value);
        drop(value);
        Some(bytes)
    }

    pub(super) fn close_bytes(value: &mut Vec<u8>, maximum_bytes: usize) -> Option<usize> {
        if value.is_empty() {
            return None;
        }
        if value.len() > maximum_bytes {
            return Some(0);
        }
        let value = std::mem::take(value);
        let bytes = value.len();
        drop(value);
        Some(bytes)
    }

    pub(in super::super) fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize) {
        if let Some(bytes) = self.rejected.as_ref() {
            if bytes.len() > maximum_bytes {
                return (false, 0);
            }
            let bytes = self.rejected.take().expect("rejected archive field remains retained");
            let released = bytes.len();
            drop(bytes);
            return (false, released);
        }
        for field in [
            &mut self.partial.reference.artifact_id,
            &mut self.partial.reference.artifact_kind,
            &mut self.partial.reference.standard,
            &mut self.partial.reference.subset,
            &mut self.partial.owner.parent.artifact_id,
            &mut self.partial.owner.parent.artifact_kind,
            &mut self.partial.owner.parent.standard,
            &mut self.partial.owner.parent.subset,
            &mut self.partial.owner.slot,
            &mut self.partial.owner.child_id,
        ] {
            if let Some(released) = Self::close_string(field, maximum_bytes) {
                return (false, released);
            }
        }
        if let Some(released) = Self::close_bytes(&mut self.partial.envelope_pack, maximum_bytes) {
            return (false, released);
        }
        if self.closing_member.is_none() {
            self.closing_member = self.archive.members.pop();
            self.close_field = 0;
        }
        if let Some(entry) = self.closing_member.as_mut() {
            let field = match self.close_field {
                0 => Some(&mut entry.reference.artifact_id),
                1 => Some(&mut entry.reference.artifact_kind),
                2 => Some(&mut entry.reference.standard),
                3 => Some(&mut entry.reference.subset),
                4 => Some(&mut entry.owner.parent.artifact_id),
                5 => Some(&mut entry.owner.parent.artifact_kind),
                6 => Some(&mut entry.owner.parent.standard),
                7 => Some(&mut entry.owner.parent.subset),
                8 => Some(&mut entry.owner.slot),
                9 => Some(&mut entry.owner.child_id),
                _ => None,
            };
            if let Some(field) = field {
                match Self::close_string(field, maximum_bytes) {
                    Some(0) => return (false, 0),
                    Some(released) => {
                        self.close_field += 1;
                        return (false, released);
                    }
                    None => {}
                }
                self.close_field += 1;
                return (false, 0);
            }
            if let Some(released) = Self::close_bytes(&mut entry.envelope_pack, maximum_bytes) {
                return (false, released);
            }
            drop(self.closing_member.take());
            return (false, 0);
        }
        if let Some(released) = Self::close_bytes(&mut self.archive.parent_spr, maximum_bytes) {
            return (false, released);
        }
        if let Some(released) = Self::close_bytes(&mut self.archive.parent_pack, maximum_bytes) {
            return (false, released);
        }
        (true, 0)
    }
}

/// 🧮️ `AppCommand::TransactionPrepare`'s `prepared_ops` roster authority — the same fixed shape
/// `DOCUMENT_ARCHIVE_MAXIMUM_MEMBERS` gives an archive's member list, declared on BOTH sides
/// (`encode_app_command` refuses to write more, [`PagedRouteFieldsDecode`] refuses to admit more) so
/// a guest never reserves an unbounded op roster out of a host-supplied count.
pub(in super::super) const TRANSACTION_PREPARED_OPS_MAXIMUM: usize = 1024;
//#endregion 🔖️PagedAppCommandDecode
