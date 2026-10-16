//! 🧩️ Paged decoders for document archives, route fields, media-export handles and `AppCommand`s over bounded command pages.

use ::protocol::wire::command_ingress::{PagedCommand, PagedCommandReader, COMMAND_MAXIMUM_BYTES};
use super::{AppCommand, DOCUMENT_ARCHIVE_MAXIMUM_MEMBERS, DocumentArchiveArtifactRef, DocumentArchiveOwnerRef, DocumentArchivePack, MediaExportHandleWire, OwnedDocumentMemberPackEntry, WindowConfigPackEntry, document_archive_into_fields};

//#region 🔖️PagedAppCommandDecode
const APP_COMMAND_FIELD_MAXIMUM_BYTES: usize = COMMAND_MAXIMUM_BYTES;

/// 🧮️ `AppCommand::PureCommand`'s two byte fields, in `encode_app_command`'s own order: `command`, then the `head`
/// snapshot pack. One field per retained decode step, so a mutation's payload never costs more than one bounded read
/// per host turn.
const PURE_COMMAND_FIELDS: usize = 2;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DocumentArchiveDecodePhase {
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
struct PartialDocumentArchiveMember {
    pub(super) ordinal: u32,
    pub(super) reference: DocumentArchiveArtifactRef,
    pub(super) owner: DocumentArchiveOwnerRef,
    pub(super) envelope_pack: Vec<u8>,
}

#[derive(Debug)]
struct PagedDocumentArchiveDecode {
    pub(super) archive: DocumentArchivePack,
    partial: PartialDocumentArchiveMember,
    pub(super) remaining: usize,
    phase: DocumentArchiveDecodePhase,
    rejected: Option<Vec<u8>>,
    closing_member: Option<OwnedDocumentMemberPackEntry>,
    close_field: u8,
}

impl PagedDocumentArchiveDecode {
    pub(super) fn new() -> Self {
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

    fn closing(archive: DocumentArchivePack) -> Self {
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

    fn read_string(&mut self, reader: &mut PagedCommandReader, label: &'static str) -> Result<String, semio_framework_diagnostic::Fault> {
        let bytes = reader.read_bounded_bytes(APP_COMMAND_FIELD_MAXIMUM_BYTES)?;
        String::from_utf8(bytes).map_err(|error| {
            self.rejected = Some(error.into_bytes());
            semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.document-archive-utf8"), label)
        })
    }

    pub(super) fn step(&mut self, reader: &mut PagedCommandReader) -> Result<Option<DocumentArchivePack>, semio_framework_diagnostic::Fault> {
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

    fn close_string(value: &mut String, maximum_bytes: usize) -> Option<usize> {
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

    fn close_bytes(value: &mut Vec<u8>, maximum_bytes: usize) -> Option<usize> {
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

    pub(super) fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize) {
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
pub(super) const TRANSACTION_PREPARED_OPS_MAXIMUM: usize = 1024;

/// 🧭️ One route-specific command's flat field plan, in `encode_app_command`'s own write order:
/// how many length-prefixed fields precede the `prepared_ops` roster, whether the roster is written
/// at all, and how many fields follow it. The decoder below and the encoder above are twins — this
/// table is the ONE place their field counts are stated, so a new field on either side is a
/// one-line change on both.
const fn route_field_plan(tag: u8) -> Option<(usize, bool, usize)> {
    match tag {
        10 => Some((3, false, 0)),
        11 => Some((2, false, 0)),
        12 => Some((1, false, 0)),
        17 => Some((3, true, 2)),
        18..=21 => Some((1, false, 0)),
        _ => None,
    }
}

/// 🧾️ The retained decoder every transaction and media route shares: one bounded field read per
/// step, an exact reservation for the `prepared_ops` roster, and a close path that releases what it
/// has accumulated one field at a time. `AppCommand::Presence` is deliberately NOT here — it is
/// admitted by its own reserved `PresenceCommandCursor` ingress (one exact page per peer), which the
/// generic cursor refuses by kind before this decoder is ever reached.
#[derive(Debug)]
struct PagedRouteFieldsDecode {
    tag: u8,
    leading: usize,
    has_ops: bool,
    trailing: usize,
    fields: Vec<Vec<u8>>,
    pub(super) ops: Vec<Vec<u8>>,
    ops_remaining: Option<usize>,
}

impl PagedRouteFieldsDecode {
    pub(super) fn new(tag: u8, plan: (usize, bool, usize)) -> Self {
        let (leading, has_ops, trailing) = plan;
        Self { tag, leading, has_ops, trailing, fields: Vec::with_capacity(leading + trailing), ops: Vec::new(), ops_remaining: None }
    }

    pub(super) fn text(fields: &mut std::vec::IntoIter<Vec<u8>>) -> Result<String, Vec<u8>> {
        let bytes = fields.next().expect("retained route field");
        String::from_utf8(bytes).map_err(std::string::FromUtf8Error::into_bytes)
    }

    pub(super) fn step(&mut self, seq: u64, reader: &mut PagedCommandReader) -> Result<Option<AppCommand>, semio_framework_diagnostic::Fault> {
        if self.fields.len() < self.leading {
            self.fields.push(reader.read_bounded_bytes(APP_COMMAND_FIELD_MAXIMUM_BYTES)?);
        } else if self.has_ops && self.ops_remaining.is_none() {
            let count = usize::try_from(reader.read_varint()?).map_err(|_| semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.transaction-prepared-ops"), "transaction prepared-ops count is not representable"))?;
            if count > TRANSACTION_PREPARED_OPS_MAXIMUM {
                return Err(semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.transaction-prepared-ops"), "transaction prepare exceeds its fixed 1024 prepared-op authority"));
            }
            self.ops.try_reserve_exact(count).map_err(|_| semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.transaction-prepared-ops-allocation"), "transaction prepared-op roster could not reserve its exact bounded authority"))?;
            self.ops_remaining = Some(count);
        } else if self.ops_remaining.is_some_and(|count| self.ops.len() < count) {
            self.ops.push(reader.read_bounded_bytes(APP_COMMAND_FIELD_MAXIMUM_BYTES)?);
        } else if self.fields.len() < self.leading + self.trailing {
            self.fields.push(reader.read_bounded_bytes(APP_COMMAND_FIELD_MAXIMUM_BYTES)?);
        }
        if self.fields.len() < self.leading + self.trailing || (self.has_ops && !self.ops_remaining.is_some_and(|count| self.ops.len() == count)) {
            return Ok(None);
        }
        Ok(Some(self.finish(seq)?))
    }

    pub(super) fn finish(&mut self, seq: u64) -> Result<AppCommand, semio_framework_diagnostic::Fault> {
        let ops = std::mem::take(&mut self.ops);
        let mut fields = std::mem::take(&mut self.fields).into_iter();
        let built = match self.tag {
            10 => Self::text(&mut fields).map(|port| AppCommand::MediaIn { seq, port, descriptor: fields.next().expect("retained media descriptor"), data: fields.next().expect("retained media data") }),
            11 => Self::text(&mut fields).map(|port| AppCommand::MediaOut { seq, port, request: fields.next().expect("retained media request") }),
            12 => Self::text(&mut fields).map(|port| AppCommand::MediaFingerprint { seq, port }),
            17 => Self::text(&mut fields).and_then(|txn_id| {
                let mutation_id = Self::text(&mut fields)?;
                let payload = fields.next().expect("retained transaction payload");
                let origin = fields.next().expect("retained transaction origin");
                Ok(AppCommand::TransactionPrepare { seq, txn_id, mutation_id, payload, prepared_ops: ops, origin, prepared_child_ops: fields.next().expect("retained transaction child op groups") })
            }),
            18 => Self::text(&mut fields).map(|txn_id| AppCommand::TransactionCommit { seq, txn_id }),
            19 => Self::text(&mut fields).map(|txn_id| AppCommand::TransactionRollback { seq, txn_id }),
            20 => Self::text(&mut fields).map(|group_id| AppCommand::TransactionUndo { seq, group_id }),
            21 => Self::text(&mut fields).map(|group_id| AppCommand::TransactionRedo { seq, group_id }),
            _ => unreachable!("route field plan admitted this tag exactly"),
        };
        match built {
            Ok(command) => Ok(command),
            Err(rejected) => {
                self.fields = std::iter::once(rejected).chain(fields).collect();
                Err(semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-field-utf8"), "route command identity is not valid UTF-8"))
            }
        }
    }

    pub(super) fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize) {
        for retained in [&mut self.ops, &mut self.fields] {
            if let Some(bytes) = retained.last() {
                if bytes.len() > maximum_bytes {
                    return (false, 0);
                }
                let bytes = retained.pop().expect("retained route field was present");
                let released = bytes.len();
                drop(bytes);
                return (false, released);
            }
        }
        (true, 0)
    }
}

#[derive(Debug, Default)]
struct PagedMediaExportHandleDecode {
    stage: u8,
    pub(super) app_instance_id: u32,
    pub(super) parent_document_id: Option<String>,
    pub(super) operation_id: u64,
    pub(super) base_revision: u64,
    rejected: Option<Vec<u8>>,
}

impl PagedMediaExportHandleDecode {
    pub(super) fn step(&mut self, reader: &mut PagedCommandReader) -> Result<Option<MediaExportHandleWire>, semio_framework_diagnostic::Fault> {
        match self.stage {
            0 => {
                self.app_instance_id = u32::try_from(reader.read_varint()?).map_err(|_| semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.media-export-app-instance"), "media export app instance exceeds u32"))?;
                self.stage = 1;
                Ok(None)
            }
            1 => {
                let bytes = reader.read_bounded_bytes(APP_COMMAND_FIELD_MAXIMUM_BYTES)?;
                self.parent_document_id = match String::from_utf8(bytes) {
                    Ok(value) => Some(value),
                    Err(error) => {
                        self.rejected = Some(error.into_bytes());
                        return Err(semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-field-utf8"), "media export parent document id is not valid UTF-8"));
                    }
                };
                self.stage = 2;
                Ok(None)
            }
            2 => {
                self.operation_id = reader.read_varint()?;
                self.stage = 3;
                Ok(None)
            }
            3 => {
                self.base_revision = reader.read_varint()?;
                self.stage = 4;
                Ok(None)
            }
            4 => Ok(Some(MediaExportHandleWire {
                app_instance_id: self.app_instance_id,
                parent_document_id: self.parent_document_id.take().expect("retained media export parent document id"),
                operation_id: self.operation_id,
                base_revision: self.base_revision,
                generation: reader.read_varint()?,
            })),
            _ => unreachable!("media export handle decoder completed once"),
        }
    }

    pub(super) fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize) {
        if let Some(bytes) = self.rejected.as_ref() {
            if bytes.len() > maximum_bytes {
                return (false, 0);
            }
            let bytes = self.rejected.take().expect("rejected media export field");
            let released = bytes.len();
            drop(bytes);
            return (false, released);
        }
        let Some(parent_document_id) = self.parent_document_id.take() else { return (true, 0) };
        let bytes = parent_document_id.into_bytes();
        if bytes.len() > maximum_bytes {
            self.parent_document_id = Some(String::from_utf8(bytes).expect("retained media export document id remains UTF-8"));
            return (false, 0);
        }
        let released = bytes.len();
        drop(bytes);
        (false, released)
    }
}

#[derive(Debug)]
enum PagedAppCommandDecodeState {
    Header,
    ConfigCommand { seq: u64 },
    CommandPayload { seq: u64 },
    CommandView { seq: u64, command: Option<Vec<u8>> },
    CommandText { seq: u64 },
    ContextMenu { seq: u64 },
    ArtifactCommand { seq: u64 },
    ReadDocument { seq: u64 },
    LoadConfigPack { seq: u64 },
    LoadConfigSpr { seq: u64, pack: Option<Vec<u8>> },
    ReadConfig { seq: u64 },
    ReadChildren { seq: u64 },
    ReadHistory { seq: u64 },
    ReadConflicts { seq: u64 },
    LocalInteractionQuery { seq: u64 },
    LoadWindowConfigWindowId { seq: u64 },
    LoadWindowConfigKind { seq: u64, window_id: Option<String> },
    LoadWindowConfigPack { seq: u64, window_id: Option<String>, window_kind_id: Option<String> },
    ReadWindowConfigs { seq: u64 },
    PureCommandFields { seq: u64, fields: Vec<Vec<u8>> },
    RouteFields { seq: u64, decode: PagedRouteFieldsDecode },
    ReadDocumentArchive { seq: u64 },
    ReadDocumentIdentity { seq: u64 },
    ReadChildHeads { seq: u64 },
    LoadDocumentArchive { seq: u64, merge: bool, decode: PagedDocumentArchiveDecode },
    DocumentArchiveOperation { seq: u64, kind: u8 },
    MediaExportSubmitPort { seq: u64 },
    MediaExportSubmitDocument { seq: u64, port: Option<String> },
    MediaExportSubmitRevision { seq: u64, port: Option<String>, expected_parent_document_id: Option<String> },
    MediaExportHandle { seq: u64, kind: u8, decode: PagedMediaExportHandleDecode },
    RejectedFields { fields: Vec<Vec<u8>> },
    Terminal,
    Faulted,
}

#[derive(Debug)]
pub struct PagedAppCommandDecodeCursor {
    reader: PagedCommandReader,
    pub(super) state: PagedAppCommandDecodeState,
}

pub struct DecodedAppCommandOwner {
    pub(super) command: Option<AppCommand>,
    close_stage: u8,
    archive_close: Option<PagedDocumentArchiveDecode>,
    text_close:Option<semio_framework_value::retirement::controlled::ControlledRetirement<String>>,
    bytes_close:Option<semio_framework_value::retirement::controlled::ControlledRetirement<Vec<u8>>>,
}

impl std::fmt::Debug for DecodedAppCommandOwner{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{f.debug_struct("DecodedAppCommandOwner").field("command",&self.command).field("close_stage",&self.close_stage).field("archive_close",&self.archive_close).field("text_close",&self.text_close.is_some()).finish()}}

impl DecodedAppCommandOwner {
    pub fn new(command: AppCommand) -> Self {
        Self { command: Some(command), close_stage: 0, archive_close: None,text_close:None,bytes_close:None }
    }

    pub fn original_command(&self) -> Option<&AppCommand> { self.command.as_ref() }
    pub fn original_command_mut(&mut self) -> Option<&mut AppCommand> { self.command.as_mut() }

    pub fn close_original_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(false)}
        if let Some(owner)=self.text_close.as_mut(){
            if owner.terminal_is_empty(){drop(self.text_close.take());cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);return Ok(false)}
            if grant.maximum_copy_bytes<owner.next_copy_byte_demand()?||grant.maximum_capacity_bytes<owner.next_capacity_byte_demand(grant.maximum_copy_bytes)?||grant.maximum_release_bytes<owner.next_release_byte_demand()?||grant.maximum_depth<owner.next_depth_demand()?{return Ok(false)}
            let step=owner.step(grant)?;cx.consume_retained(step.progress())?;cx.consume_fuel(step.progress().copied_items as u64);return Ok(false)
        }
        if let Some(owner)=self.bytes_close.as_mut(){
            if owner.terminal_is_empty(){drop(self.bytes_close.take());cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);return Ok(false)}
            if grant.maximum_copy_bytes<owner.next_copy_byte_demand()?||grant.maximum_capacity_bytes<owner.next_capacity_byte_demand(grant.maximum_copy_bytes)?||grant.maximum_release_bytes<owner.next_release_byte_demand()?||grant.maximum_depth<owner.next_depth_demand()?{return Ok(false)}
            let step=owner.step(grant)?;cx.consume_retained(step.progress())?;cx.consume_fuel(step.progress().copied_items as u64);return Ok(false)
        }
        let Some(command)=self.command.as_mut()else{return Ok(true)};
        let bytes = match (self.close_stage, &mut *command) {
            (0, AppCommand::MediaOut { request, .. }) => Some(request),
            (0, AppCommand::Command { command, .. } | AppCommand::ArtifactCommand { command, .. }) => Some(command),
            (1, AppCommand::Command { view_state, .. }) => Some(view_state),
            _ => None,
        };
        if let Some(bytes) = bytes {
            use semio_framework_value::retirement::RetireOwned as _;
            let birth = bytes.retirement_birth_bytes().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "original command bytes have no retirement factory"))?;
            let copy = size_of::<Vec<u8>>();
            if grant.maximum_copy_bytes < copy || grant.maximum_capacity_bytes < birth { return Ok(false) }
            match semio_framework_value::retirement::controlled::ControlledRetirement::new(std::mem::take(bytes)) {
                Ok(owner) => { self.bytes_close = Some(owner); self.close_stage += 1; }
                Err((failure, source)) => { *bytes = source; return Err(failure.into()) }
            }
            cx.consume_retained(semio_framework_value::RetainedCloneProgress { copied_items: 1, copied_bytes: copy, retained_capacity_bytes: birth, ..Default::default() })?;
            cx.consume_fuel(1);
            return Ok(false)
        }
        let field=match(self.close_stage,&mut*command){
            (0,AppCommand::SubmitMediaExport{port,..})=>Some(port),
            (1,AppCommand::SubmitMediaExport{expected_parent_document_id,..})=>Some(expected_parent_document_id),
            (0,AppCommand::PollMediaExport{handle,..})=>Some(&mut handle.parent_document_id),
            (1,AppCommand::MediaOut{port,..})=>Some(port),
            (_,AppCommand::SubmitMediaExport{..}|AppCommand::PollMediaExport{..}|AppCommand::MediaOut{..}|AppCommand::Command{..}|AppCommand::ArtifactCommand{..})=>None,
            _=>return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"original command close requires an admitted payload carrier").into()),
        };
        if let Some(field)=field{
            match semio_framework_value::retirement::controlled::ControlledRetirement::new(std::mem::take(field)){Ok(owner)=>{self.text_close=Some(owner);self.close_stage+=1;},Err((failure,source))=>{*field=source;return Err(failure.into())}}
        }else{drop(self.command.take());}
        cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);Ok(self.command.is_none())
    }

    pub fn take_for_dispatch(&mut self) -> Option<AppCommand> {
        self.command.take()
    }

    pub fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
        let Some(command) = self.command.as_mut() else {
            return (true, 0, 0);
        };
        // 🔒️ `close_stage == 0` is the latch. Without it a drained archive was rebuilt from the
        // emptied `DocumentArchivePack` on the very next turn, reported terminal, was dropped, and
        // was rebuilt again — so `LoadDocumentArchive` never reached the shell release below and no
        // decoded archive owner ever became terminal.
        if self.archive_close.is_none() && self.close_stage == 0 {
            if let AppCommand::LoadDocumentArchive { archive, .. } | AppCommand::MergeDocumentArchive { archive, .. } = command {
                self.archive_close = Some(PagedDocumentArchiveDecode::closing(std::mem::take(archive)));
            }
        }
        if let Some(archive) = self.archive_close.as_mut() {
            let (empty, released) = archive.close_step(maximum_bytes);
            if !empty || released != 0 {
                return (false, usize::from(released != 0), released);
            }
            drop(self.archive_close.take());
            self.close_stage = self.close_stage.saturating_add(1);
            return (false, 1, 0);
        }
        // 🧾️ A prepared-op roster is a LIST, not a fixed field slot, so it drains before the flat
        // stages below rather than inside them — one op per step, and an op too large for this
        // step's grant stays retained instead of being released unbounded.
        if let AppCommand::TransactionPrepare { prepared_ops, .. } = command {
            if let Some(op) = prepared_ops.last() {
                if op.len() > maximum_bytes {
                    return (false, 0, 0);
                }
                let op = prepared_ops.pop().expect("retained prepared op was present");
                let released = op.len();
                drop(op);
                return (false, 1, released);
            }
        }
        let field = match (self.close_stage, command) {
            (0, AppCommand::ConfigCommand { command, .. }) | (0, AppCommand::ContextMenu { request: command, .. }) | (0, AppCommand::ArtifactCommand { command, .. }) | (0, AppCommand::Command { command, .. }) => Some(std::mem::take(command)),
            (0, AppCommand::CommandText { line, .. }) => Some(std::mem::take(line).into_bytes()),
            (0, AppCommand::LoadConfig { pack, .. }) => Some(std::mem::take(pack)),
            (0, AppCommand::LoadWindowConfig { entry, .. }) => Some(std::mem::take(&mut entry.window_id).into_bytes()),
            (1, AppCommand::Command { view_state, .. }) => Some(std::mem::take(view_state)),
            (1, AppCommand::LoadConfig { spr, .. }) => Some(std::mem::take(spr)),
            (1, AppCommand::LoadWindowConfig { entry, .. }) => Some(std::mem::take(&mut entry.window_kind_id).into_bytes()),
            (2, AppCommand::LoadWindowConfig { entry, .. }) => Some(std::mem::take(&mut entry.envelope_pack)),
            (0, AppCommand::MediaIn { data, .. }) => Some(std::mem::take(data)),
            (1, AppCommand::MediaIn { descriptor, .. }) => Some(std::mem::take(descriptor)),
            (2, AppCommand::MediaIn { port, .. }) => Some(std::mem::take(port).into_bytes()),
            (0, AppCommand::MediaOut { request, .. }) => Some(std::mem::take(request)),
            (1, AppCommand::MediaOut { port, .. }) | (0, AppCommand::MediaFingerprint { port, .. }) => Some(std::mem::take(port).into_bytes()),
            (0, AppCommand::TransactionPrepare { payload, .. }) => Some(std::mem::take(payload)),
            (1, AppCommand::TransactionPrepare { origin, .. }) => Some(std::mem::take(origin)),
            (2, AppCommand::TransactionPrepare { txn_id, .. }) => Some(std::mem::take(txn_id).into_bytes()),
            (3, AppCommand::TransactionPrepare { mutation_id, .. }) => Some(std::mem::take(mutation_id).into_bytes()),
            (4, AppCommand::TransactionPrepare { prepared_child_ops, .. }) => Some(std::mem::take(prepared_child_ops)),
            (0, AppCommand::TransactionCommit { txn_id, .. }) | (0, AppCommand::TransactionRollback { txn_id, .. }) => Some(std::mem::take(txn_id).into_bytes()),
            (0, AppCommand::TransactionUndo { group_id, .. }) | (0, AppCommand::TransactionRedo { group_id, .. }) => Some(std::mem::take(group_id).into_bytes()),
            (0, AppCommand::SubmitMediaExport { port, .. }) => Some(std::mem::take(port).into_bytes()),
            (1, AppCommand::SubmitMediaExport { expected_parent_document_id, .. }) => Some(std::mem::take(expected_parent_document_id).into_bytes()),
            (0, AppCommand::PollMediaExport { handle, .. }) | (0, AppCommand::CancelMediaExport { handle, .. }) | (0, AppCommand::TakeMediaExportChunk { handle, .. }) => Some(std::mem::take(&mut handle.parent_document_id).into_bytes()),
            _ => None,
        };
        if let Some(field) = field {
            if field.len() > maximum_bytes {
                match (self.close_stage, self.command.as_mut().expect("decoded command owner was present")) {
                    (0, AppCommand::ConfigCommand { command, .. }) | (0, AppCommand::ContextMenu { request: command, .. }) | (0, AppCommand::ArtifactCommand { command, .. }) | (0, AppCommand::Command { command, .. }) => *command = field,
                    (0, AppCommand::CommandText { line, .. }) => *line = String::from_utf8(field).expect("decoded command text remains valid UTF-8"),
                    (0, AppCommand::LoadConfig { pack, .. }) => *pack = field,
                    (0, AppCommand::LoadWindowConfig { entry, .. }) => entry.window_id = String::from_utf8(field).expect("decoded window id remains valid UTF-8"),
                    (1, AppCommand::Command { view_state, .. }) => *view_state = field,
                    (1, AppCommand::LoadConfig { spr, .. }) => *spr = field,
                    (1, AppCommand::LoadWindowConfig { entry, .. }) => entry.window_kind_id = String::from_utf8(field).expect("decoded window kind id remains valid UTF-8"),
                    (2, AppCommand::LoadWindowConfig { entry, .. }) => entry.envelope_pack = field,
                    (0, AppCommand::MediaIn { data, .. }) => *data = field,
                    (1, AppCommand::MediaIn { descriptor, .. }) => *descriptor = field,
                    (2, AppCommand::MediaIn { port, .. }) => *port = String::from_utf8(field).expect("decoded media port remains valid UTF-8"),
                    (0, AppCommand::MediaOut { request, .. }) => *request = field,
                    (1, AppCommand::MediaOut { port, .. }) | (0, AppCommand::MediaFingerprint { port, .. }) => *port = String::from_utf8(field).expect("decoded media port remains valid UTF-8"),
                    (0, AppCommand::TransactionPrepare { payload, .. }) => *payload = field,
                    (1, AppCommand::TransactionPrepare { origin, .. }) => *origin = field,
                    (2, AppCommand::TransactionPrepare { txn_id, .. }) => *txn_id = String::from_utf8(field).expect("decoded transaction id remains valid UTF-8"),
                    (3, AppCommand::TransactionPrepare { mutation_id, .. }) => *mutation_id = String::from_utf8(field).expect("decoded mutation id remains valid UTF-8"),
                    (4, AppCommand::TransactionPrepare { prepared_child_ops, .. }) => *prepared_child_ops = field,
                    (0, AppCommand::TransactionCommit { txn_id, .. }) | (0, AppCommand::TransactionRollback { txn_id, .. }) => *txn_id = String::from_utf8(field).expect("decoded transaction id remains valid UTF-8"),
                    (0, AppCommand::TransactionUndo { group_id, .. }) | (0, AppCommand::TransactionRedo { group_id, .. }) => *group_id = String::from_utf8(field).expect("decoded transaction group id remains valid UTF-8"),
                    (0, AppCommand::SubmitMediaExport { port, .. }) => *port = String::from_utf8(field).expect("decoded media export port remains valid UTF-8"),
                    (1, AppCommand::SubmitMediaExport { expected_parent_document_id, .. }) => *expected_parent_document_id = String::from_utf8(field).expect("decoded media export parent document id remains valid UTF-8"),
                    (0, AppCommand::PollMediaExport { handle, .. }) | (0, AppCommand::CancelMediaExport { handle, .. }) | (0, AppCommand::TakeMediaExportChunk { handle, .. }) => handle.parent_document_id = String::from_utf8(field).expect("decoded media export parent document id remains valid UTF-8"),
                    _ => unreachable!("decoded command close field has an exact restoration target"),
                }
                return (false, 0, 0);
            }
            let released = field.len();
            drop(field);
            self.close_stage = self.close_stage.saturating_add(1);
            return (false, 1, released);
        }
        let command = self.command.take().expect("decoded command shell was present");
        drop(command);
        (true, 1, 0)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.command.is_none()&&self.text_close.is_none()&&self.bytes_close.is_none()
    }
}

/// 🧭️ Which page index an ingress cursor is expected to carry — see `validate_cursor`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CursorShape {
    Page,
    Terminal,
    Fault,
}

impl PagedAppCommandDecodeCursor {
    pub fn new(command: PagedCommand) -> Self {
        Self { reader: PagedCommandReader::new(command), state: PagedAppCommandDecodeState::Header }
    }

    pub fn kind(&self) -> u8 {
        self.reader.kind()
    }

    pub fn step(&mut self) -> Result<Option<AppCommand>, semio_framework_diagnostic::Fault> {
        let state = std::mem::replace(&mut self.state, PagedAppCommandDecodeState::Faulted);
        let outcome = match state {
            PagedAppCommandDecodeState::Header => {
                let tag = self.reader.read_byte()?;
                let seq = self.reader.read_varint()?;
                self.state = match tag {
                    0 => PagedAppCommandDecodeState::ConfigCommand { seq },
                    1 => PagedAppCommandDecodeState::CommandPayload { seq },
                    2 => PagedAppCommandDecodeState::CommandText { seq },
                    3 => PagedAppCommandDecodeState::ContextMenu { seq },
                    4 => PagedAppCommandDecodeState::ArtifactCommand { seq },
                    7 => PagedAppCommandDecodeState::ReadDocument { seq },
                    8 => PagedAppCommandDecodeState::LoadConfigPack { seq },
                    9 => PagedAppCommandDecodeState::ReadConfig { seq },
                    13 => PagedAppCommandDecodeState::PureCommandFields { seq, fields: Vec::with_capacity(PURE_COMMAND_FIELDS) },
                    15 => PagedAppCommandDecodeState::ReadChildren { seq },
                    16 => PagedAppCommandDecodeState::ReadHistory { seq },
                    27 => PagedAppCommandDecodeState::ReadConflicts { seq },
                    29 => PagedAppCommandDecodeState::LocalInteractionQuery { seq },
                    30 => PagedAppCommandDecodeState::LoadWindowConfigWindowId { seq },
                    31 => PagedAppCommandDecodeState::ReadWindowConfigs { seq },
                    32 | 43 => PagedAppCommandDecodeState::LoadDocumentArchive { seq, merge: tag == 43, decode: PagedDocumentArchiveDecode::new() },
                    33 => PagedAppCommandDecodeState::ReadDocumentArchive { seq },
                    41 => PagedAppCommandDecodeState::ReadDocumentIdentity { seq },
                    42 => PagedAppCommandDecodeState::ReadChildHeads { seq },
                    34..=36 => PagedAppCommandDecodeState::DocumentArchiveOperation { seq, kind: tag },
                    37 => PagedAppCommandDecodeState::MediaExportSubmitPort { seq },
                    38..=40 => PagedAppCommandDecodeState::MediaExportHandle { seq, kind: tag, decode: PagedMediaExportHandleDecode::default() },
                    tag if route_field_plan(tag).is_some() => {
                        PagedAppCommandDecodeState::RouteFields { seq, decode: PagedRouteFieldsDecode::new(tag, route_field_plan(tag).expect("route field plan was just matched")) }
                    }
                    _ => {
                        return Err(semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-route-state-machine-required"), "this AppCommand kind requires its route-specific retained decoder before admission"));
                    }
                };
                None
            }
            PagedAppCommandDecodeState::ConfigCommand { seq } => {
                let command = self.reader.read_bounded_bytes(APP_COMMAND_FIELD_MAXIMUM_BYTES)?;
                Some(AppCommand::ConfigCommand { seq, command })
            }
            PagedAppCommandDecodeState::CommandPayload { seq } => {
                let command = self.reader.read_bounded_bytes(APP_COMMAND_FIELD_MAXIMUM_BYTES)?;
                self.state = PagedAppCommandDecodeState::CommandView { seq, command: Some(command) };
                None
            }
            PagedAppCommandDecodeState::CommandView { seq, mut command } => {
                let view_state = match self.reader.read_bounded_bytes(APP_COMMAND_FIELD_MAXIMUM_BYTES) {
                    Ok(view_state) => view_state,
                    Err(fault) => {
                        self.state = PagedAppCommandDecodeState::CommandView { seq, command };
                        return Err(fault);
                    }
                };
                Some(AppCommand::Command { seq, command: command.take().expect("retained command payload"), view_state })
            }
            PagedAppCommandDecodeState::CommandText { seq } => {
                let bytes = self.reader.read_bounded_bytes(APP_COMMAND_FIELD_MAXIMUM_BYTES)?;
                let line = match String::from_utf8(bytes) {
                    Ok(line) => line,
                    Err(error) => {
                        self.state = PagedAppCommandDecodeState::RejectedFields { fields: vec![error.into_bytes()] };
                        return Err(semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-field-utf8"), "paged command text is not valid UTF-8"));
                    }
                };
                Some(AppCommand::CommandText { seq, line })
            }
            PagedAppCommandDecodeState::ContextMenu { seq } => {
                let request = self.reader.read_bounded_bytes(APP_COMMAND_FIELD_MAXIMUM_BYTES)?;
                Some(AppCommand::ContextMenu { seq, request })
            }
            PagedAppCommandDecodeState::ArtifactCommand { seq } => {
                let command = self.reader.read_bounded_bytes(APP_COMMAND_FIELD_MAXIMUM_BYTES)?;
                Some(AppCommand::ArtifactCommand { seq, command })
            }
            PagedAppCommandDecodeState::ReadDocument { seq } => Some(AppCommand::ReadDocument { seq }),
            PagedAppCommandDecodeState::LoadConfigPack { seq } => {
                let pack = self.reader.read_bounded_bytes(APP_COMMAND_FIELD_MAXIMUM_BYTES)?;
                self.state = PagedAppCommandDecodeState::LoadConfigSpr { seq, pack: Some(pack) };
                None
            }
            PagedAppCommandDecodeState::LoadConfigSpr { seq, mut pack } => {
                let spr = match self.reader.read_bounded_bytes(APP_COMMAND_FIELD_MAXIMUM_BYTES) {
                    Ok(spr) => spr,
                    Err(fault) => {
                        self.state = PagedAppCommandDecodeState::LoadConfigSpr { seq, pack };
                        return Err(fault);
                    }
                };
                Some(AppCommand::LoadConfig { seq, pack: pack.take().expect("retained config pack"), spr })
            }
            PagedAppCommandDecodeState::ReadConfig { seq } => Some(AppCommand::ReadConfig { seq }),
            PagedAppCommandDecodeState::ReadChildren { seq } => Some(AppCommand::ReadChildren { seq }),
            PagedAppCommandDecodeState::ReadHistory { seq } => Some(AppCommand::ReadHistory { seq }),
            PagedAppCommandDecodeState::ReadConflicts { seq } => Some(AppCommand::ReadConflicts { seq }),
            PagedAppCommandDecodeState::ReadDocumentIdentity { seq } => Some(AppCommand::ReadDocumentIdentity { seq }),
            PagedAppCommandDecodeState::ReadChildHeads { seq } => Some(AppCommand::ReadChildHeads { seq }),
            PagedAppCommandDecodeState::LocalInteractionQuery { seq } => {
                let bytes = self.reader.read_bounded_bytes(142)?;
                let command = protocol::decode_local_interaction_query_command(&bytes).map_err(|reason| semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("local-interaction.command-wire"), reason))?;
                Some(AppCommand::LocalInteractionQuery { seq, command })
            }
            PagedAppCommandDecodeState::LoadWindowConfigWindowId { seq } => {
                let bytes = self.reader.read_bounded_bytes(APP_COMMAND_FIELD_MAXIMUM_BYTES)?;
                let window_id = match String::from_utf8(bytes) {
                    Ok(window_id) => window_id,
                    Err(error) => {
                        self.state = PagedAppCommandDecodeState::RejectedFields { fields: vec![error.into_bytes()] };
                        return Err(semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-field-utf8"), "window config window id is not valid UTF-8"));
                    }
                };
                self.state = PagedAppCommandDecodeState::LoadWindowConfigKind { seq, window_id: Some(window_id) };
                None
            }
            PagedAppCommandDecodeState::LoadWindowConfigKind { seq, mut window_id } => {
                let bytes = match self.reader.read_bounded_bytes(APP_COMMAND_FIELD_MAXIMUM_BYTES) {
                    Ok(bytes) => bytes,
                    Err(fault) => {
                        self.state = PagedAppCommandDecodeState::LoadWindowConfigKind { seq, window_id };
                        return Err(fault);
                    }
                };
                let window_kind_id = match String::from_utf8(bytes) {
                    Ok(window_kind_id) => window_kind_id,
                    Err(error) => {
                        self.state = PagedAppCommandDecodeState::RejectedFields {
                            fields: vec![window_id.take().expect("retained window id").into_bytes(), error.into_bytes()],
                        };
                        return Err(semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-field-utf8"), "window config kind id is not valid UTF-8"));
                    }
                };
                self.state = PagedAppCommandDecodeState::LoadWindowConfigPack { seq, window_id, window_kind_id: Some(window_kind_id) };
                None
            }
            PagedAppCommandDecodeState::LoadWindowConfigPack { seq, mut window_id, mut window_kind_id } => {
                let envelope_pack = match self.reader.read_bounded_bytes(APP_COMMAND_FIELD_MAXIMUM_BYTES) {
                    Ok(envelope_pack) => envelope_pack,
                    Err(fault) => {
                        self.state = PagedAppCommandDecodeState::LoadWindowConfigPack { seq, window_id, window_kind_id };
                        return Err(fault);
                    }
                };
                Some(AppCommand::LoadWindowConfig {
                    seq,
                    entry: WindowConfigPackEntry {
                        window_id: window_id.take().expect("retained window id"),
                        window_kind_id: window_kind_id.take().expect("retained window kind id"),
                        envelope_pack,
                    },
                })
            }
            PagedAppCommandDecodeState::ReadWindowConfigs { seq } => Some(AppCommand::ReadWindowConfigs { seq }),
            PagedAppCommandDecodeState::PureCommandFields { seq, mut fields } => {
                let field = match self.reader.read_bounded_bytes(APP_COMMAND_FIELD_MAXIMUM_BYTES) {
                    Ok(field) => field,
                    Err(fault) => {
                        self.state = PagedAppCommandDecodeState::PureCommandFields { seq, fields };
                        return Err(fault);
                    }
                };
                fields.push(field);
                if fields.len() < PURE_COMMAND_FIELDS {
                    self.state = PagedAppCommandDecodeState::PureCommandFields { seq, fields };
                    return Ok(None);
                }
                let mut fields = fields.into_iter();
                let mut next = || fields.next().expect("retained pure-command field");
                Some(AppCommand::PureCommand { seq, command: next(), head: next() })
            }
            PagedAppCommandDecodeState::RouteFields { seq, mut decode } => match decode.step(seq, &mut self.reader) {
                Ok(Some(command)) => Some(command),
                Ok(None) => {
                    self.state = PagedAppCommandDecodeState::RouteFields { seq, decode };
                    None
                }
                Err(fault) => {
                    self.state = PagedAppCommandDecodeState::RouteFields { seq, decode };
                    return Err(fault);
                }
            },
            PagedAppCommandDecodeState::ReadDocumentArchive { seq } =>Some(AppCommand::ReadDocumentArchive { seq }),
            PagedAppCommandDecodeState::LoadDocumentArchive { seq, merge, mut decode } => match decode.step(&mut self.reader) {
                Ok(Some(archive)) => Some(if merge { AppCommand::MergeDocumentArchive { seq, archive } } else { AppCommand::LoadDocumentArchive { seq, archive } }),
                Ok(None) => {
                    self.state = PagedAppCommandDecodeState::LoadDocumentArchive { seq, merge, decode };
                    None
                }
                Err(fault) => {
                    self.state = PagedAppCommandDecodeState::LoadDocumentArchive { seq, merge, decode };
                    return Err(fault);
                }
            },
            PagedAppCommandDecodeState::DocumentArchiveOperation { seq, kind } => {
                let operation = self.reader.read_varint()?;
                Some(match kind {
                    34 => AppCommand::PollDocumentArchiveLoad { seq, operation },
                    35 => AppCommand::CancelDocumentArchiveLoad { seq, operation },
                    36 => AppCommand::AcknowledgeDocumentArchiveLoad { seq, operation },
                    _ => unreachable!("archive operation command tag was admitted exactly"),
                })
            }
            PagedAppCommandDecodeState::MediaExportSubmitPort { seq } => {
                let bytes = self.reader.read_bounded_bytes(APP_COMMAND_FIELD_MAXIMUM_BYTES)?;
                let port = match String::from_utf8(bytes) {
                    Ok(value) => value,
                    Err(error) => {
                        self.state = PagedAppCommandDecodeState::RejectedFields { fields: vec![error.into_bytes()] };
                        return Err(semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-field-utf8"), "media export port is not valid UTF-8"));
                    }
                };
                self.state = PagedAppCommandDecodeState::MediaExportSubmitDocument { seq, port: Some(port) };
                None
            }
            PagedAppCommandDecodeState::MediaExportSubmitDocument { seq, mut port } => {
                let bytes = match self.reader.read_bounded_bytes(APP_COMMAND_FIELD_MAXIMUM_BYTES) {
                    Ok(bytes) => bytes,
                    Err(fault) => {
                        self.state = PagedAppCommandDecodeState::MediaExportSubmitDocument { seq, port };
                        return Err(fault);
                    }
                };
                let expected_parent_document_id = match String::from_utf8(bytes) {
                    Ok(value) => value,
                    Err(error) => {
                        self.state = PagedAppCommandDecodeState::RejectedFields { fields: vec![port.take().expect("retained media export port").into_bytes(), error.into_bytes()] };
                        return Err(semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-field-utf8"), "media export parent document id is not valid UTF-8"));
                    }
                };
                self.state = PagedAppCommandDecodeState::MediaExportSubmitRevision { seq, port, expected_parent_document_id: Some(expected_parent_document_id) };
                None
            }
            PagedAppCommandDecodeState::MediaExportSubmitRevision { seq, mut port, mut expected_parent_document_id } => Some(AppCommand::SubmitMediaExport {
                seq,
                port: port.take().expect("retained media export port"),
                expected_parent_document_id: expected_parent_document_id.take().expect("retained media export parent document id"),
                expected_base_revision: self.reader.read_varint()?,
            }),
            PagedAppCommandDecodeState::MediaExportHandle { seq, kind, mut decode } => match decode.step(&mut self.reader) {
                Ok(Some(handle)) => Some(match kind {
                    38 => AppCommand::PollMediaExport { seq, handle },
                    39 => AppCommand::CancelMediaExport { seq, handle },
                    40 => AppCommand::TakeMediaExportChunk { seq, handle },
                    _ => unreachable!("media export operation tag was admitted exactly"),
                }),
                Ok(None) => {
                    self.state = PagedAppCommandDecodeState::MediaExportHandle { seq, kind, decode };
                    None
                }
                Err(fault) => {
                    self.state = PagedAppCommandDecodeState::MediaExportHandle { seq, kind, decode };
                    return Err(fault);
                }
            },
            PagedAppCommandDecodeState::RejectedFields { fields } => {
                self.state = PagedAppCommandDecodeState::RejectedFields { fields };
                return Err(semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-decode-closing"), "rejected paged command must be closed before it can be stepped again"));
            }
            PagedAppCommandDecodeState::Terminal | PagedAppCommandDecodeState::Faulted => {
                return Err(semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-decode-terminal"), "paged command decoder was stepped after terminal"));
            }
        };
        if let Some(command) = outcome {
            if !self.reader.terminal_is_empty() {
                let fields = match command {
                    AppCommand::ConfigCommand { command, .. } | AppCommand::ContextMenu { request: command, .. } | AppCommand::ArtifactCommand { command, .. } => vec![command],
                    AppCommand::Command { command, view_state, .. } => vec![command, view_state],
                    AppCommand::CommandText { line, .. } => vec![line.into_bytes()],
                    AppCommand::LoadConfig { pack, spr, .. } => vec![pack, spr],
                    AppCommand::PureCommand { command, head, .. } => vec![command, head],
                    AppCommand::LoadWindowConfig { entry, .. } => vec![entry.window_id.into_bytes(), entry.window_kind_id.into_bytes(), entry.envelope_pack],
                    AppCommand::ReadDocument { .. }
                    | AppCommand::ReadConfig { .. }
                    | AppCommand::ReadChildren { .. }
                    | AppCommand::ReadHistory { .. }
                    | AppCommand::ReadConflicts { .. }
                    | AppCommand::LocalInteractionQuery { .. }
                    | AppCommand::ReadDocumentIdentity { .. }
                    | AppCommand::ReadChildHeads { .. }
                    | AppCommand::ReadWindowConfigs { .. }
                    | AppCommand::PollDocumentArchiveLoad { .. }
                    | AppCommand::CancelDocumentArchiveLoad { .. }
                    | AppCommand::AcknowledgeDocumentArchiveLoad { .. } => Vec::new(),
                    AppCommand::ReadDocumentArchive { .. } => Vec::new(),
                    AppCommand::LoadDocumentArchive { archive, .. } | AppCommand::MergeDocumentArchive { archive, .. } => document_archive_into_fields(archive),
                    AppCommand::MediaIn { port, descriptor, data, .. } => vec![port.into_bytes(), descriptor, data],
                    AppCommand::MediaOut { port, request, .. } => vec![port.into_bytes(), request],
                    AppCommand::MediaFingerprint { port, .. } => vec![port.into_bytes()],
                    AppCommand::TransactionPrepare { txn_id, mutation_id, payload, prepared_ops, origin, prepared_child_ops, .. } => {
                        let mut fields = vec![txn_id.into_bytes(), mutation_id.into_bytes(), payload, origin, prepared_child_ops];
                        fields.extend(prepared_ops);
                        fields
                    }
                    AppCommand::TransactionCommit { txn_id, .. } | AppCommand::TransactionRollback { txn_id, .. } => vec![txn_id.into_bytes()],
                    AppCommand::TransactionUndo { group_id, .. } | AppCommand::TransactionRedo { group_id, .. } => vec![group_id.into_bytes()],
                    AppCommand::SubmitMediaExport { port, expected_parent_document_id, .. } => vec![port.into_bytes(), expected_parent_document_id.into_bytes()],
                    AppCommand::PollMediaExport { handle, .. } | AppCommand::CancelMediaExport { handle, .. } | AppCommand::TakeMediaExportChunk { handle, .. } => vec![handle.parent_document_id.into_bytes()],
                    AppCommand::Presence { .. } => unreachable!("Presence is never decoded by the generic paged cursor"),
                    _ => unreachable!("route-specific AppCommand is never decoded by the generic paged cursor"),
                };
                self.state = PagedAppCommandDecodeState::RejectedFields { fields };
                return Err(semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-decode-trailing"), "paged command carries trailing bytes after its terminal field"));
            }
            self.state = PagedAppCommandDecodeState::Terminal;
            Ok(Some(command))
        } else {
            Ok(None)
        }
    }

    pub fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize) {
        if let PagedAppCommandDecodeState::LoadDocumentArchive { decode, .. } = &mut self.state {
            let (empty, released) = decode.close_step(maximum_bytes);
            if !empty || released != 0 {
                return (false, released);
            }
        }
        if let PagedAppCommandDecodeState::RouteFields { decode, .. } = &mut self.state {
            let (empty, released) = decode.close_step(maximum_bytes);
            if !empty || released != 0 {
                return (false, released);
            }
        }
        if let PagedAppCommandDecodeState::MediaExportHandle { decode, .. } = &mut self.state {
            let (empty, released) = decode.close_step(maximum_bytes);
            if !empty || released != 0 {
                return (false, released);
            }
        }
        if let PagedAppCommandDecodeState::CommandView { command, .. } = &mut self.state {
            if let Some(bytes) = command.as_ref() {
                if bytes.len() > maximum_bytes {
                    return (false, 0);
                }
                let bytes = command.take().expect("retained command payload");
                let released = bytes.len();
                drop(bytes);
                return (false, released);
            }
        }
        if let Some(retained) = match &mut self.state {
            PagedAppCommandDecodeState::LoadConfigSpr { pack, .. } => Some(pack),
            _ => None,
        } {
            if let Some(bytes) = retained.as_ref() {
                if bytes.len() > maximum_bytes {
                    return (false, 0);
                }
                let bytes = retained.take().expect("retained pack was present");
                let released = bytes.len();
                drop(bytes);
                return (false, released);
            }
        }
        if let PagedAppCommandDecodeState::LoadWindowConfigKind { window_id, .. } = &mut self.state {
            if let Some(text) = window_id.take() {
                let bytes = text.into_bytes();
                if bytes.len() > maximum_bytes {
                    *window_id = Some(String::from_utf8(bytes).expect("retained window config identity remains valid UTF-8"));
                    return (false, 0);
                }
                let released = bytes.len();
                drop(bytes);
                return (false, released);
            }
        }
        if let PagedAppCommandDecodeState::LoadWindowConfigPack { window_id, window_kind_id, .. } = &mut self.state {
            let retained = if window_id.is_some() { window_id } else { window_kind_id };
            if let Some(text) = retained.take() {
                let bytes = text.into_bytes();
                if bytes.len() > maximum_bytes {
                    *retained = Some(String::from_utf8(bytes).expect("retained window config identity remains valid UTF-8"));
                    return (false, 0);
                }
                let released = bytes.len();
                drop(bytes);
                return (false, released);
            }
        }
        if let Some(retained) = match &mut self.state {
            PagedAppCommandDecodeState::MediaExportSubmitDocument { port, .. } => Some(port),
            PagedAppCommandDecodeState::MediaExportSubmitRevision { port, expected_parent_document_id, .. } => Some(if port.is_some() { port } else { expected_parent_document_id }),
            _ => None,
        } {
            if let Some(text) = retained.take() {
                let bytes = text.into_bytes();
                if bytes.len() > maximum_bytes {
                    *retained = Some(String::from_utf8(bytes).expect("retained media export identity remains valid UTF-8"));
                    return (false, 0);
                }
                let released = bytes.len();
                drop(bytes);
                return (false, released);
            }
        }
        if let PagedAppCommandDecodeState::RejectedFields { fields } = &mut self.state {
            if let Some(bytes) = fields.last() {
                if bytes.len() > maximum_bytes {
                    return (false, 0);
                }
            }
            if let Some(bytes) = fields.pop() {
                let released = bytes.len();
                drop(bytes);
                return (false, released);
            }
        }
        let (empty, released) = self.reader.close_step(maximum_bytes);
        if empty {
            self.state = PagedAppCommandDecodeState::Terminal;
        }
        (empty, released)
    }

    pub fn terminal_is_empty(&self) -> bool {
        matches!(self.state, PagedAppCommandDecodeState::Terminal) && self.reader.terminal_is_empty()
    }
}
//#endregion 🔖️PagedAppCommandDecode
