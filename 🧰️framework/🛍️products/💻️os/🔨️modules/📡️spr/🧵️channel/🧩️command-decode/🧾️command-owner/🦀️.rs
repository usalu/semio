//! 🧾️ Command owner of the paged command decoders: the decode state, the cursor and owner records, the owner implementation and the cursor shape. See [`super`].

use super::*;
use std::mem::size_of;

//#region 🔖️PagedAppCommandDecode
#[derive(Debug)]
#[derive(semio_framework_value::RetireOwned)]
#[value(crate="::semio_framework_value")]
pub(super) enum PagedAppCommandDecodeState {
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
    MediaExportSubmitSelection { seq:u64,port:Option<String>,expected_parent_document_id:Option<String>,expected_base_revision:u64 },
    MediaExportSubmitCheckpoint { seq:u64,port:Option<String>,expected_parent_document_id:Option<String>,expected_base_revision:u64 },
    MediaExportHandle { seq: u64, kind: u8, decode: PagedMediaExportHandleDecode },
    RejectedFields { fields: Vec<Vec<u8>> },
    Terminal,
    Faulted,
}

pub struct PagedAppCommandDecodeCursor {
    pub(super) reader: PagedCommandReader,
    pub(in super::super) state: PagedAppCommandDecodeState,
    pub(super) original:super::original_cursor::OriginalCommandDecode,
    pub(super) original_close:Option<super::original_cursor::OriginalCursorClose>,
    pub(super) original_active:bool,
    pub(super) original_retired:bool,
}

pub struct DecodedAppCommandOwner {
    pub(in super::super) command: Option<AppCommand>,
    pub(super) close_stage: u8,
    pub(super) archive_close: Option<PagedDocumentArchiveDecode>,
    pub(super) original_close:Option<semio_framework_value::retirement::controlled::ControlledRetirement<AppCommand>>,
}

impl std::fmt::Debug for DecodedAppCommandOwner{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{f.debug_struct("DecodedAppCommandOwner").field("command",&self.command).field("close_stage",&self.close_stage).field("archive_close",&self.archive_close).field("original_close",&self.original_close.is_some()).finish()}}

impl DecodedAppCommandOwner {
    pub fn new(command: AppCommand) -> Self {
        Self { command: Some(command), close_stage: 0, archive_close: None,original_close:None }
    }

    pub fn original_command(&self) -> Option<&AppCommand> { self.command.as_ref() }
    pub fn original_command_mut(&mut self) -> Option<&mut AppCommand> { self.command.as_mut() }

    pub fn close_original_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        use semio_framework_value::{RetainedCloneProgress as P,retirement::{RetireOwned,controlled::ControlledRetirement},retained_clone::retained_clone_grant_admits};
        let grant=cx.retained_grant();if cx.retirement_should_yield(){return Ok(false)}
        if self.archive_close.is_some(){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"command retirement cannot combine two active frontiers").into())}
        if self.original_close.is_none(){
            if self.command.is_none(){return Ok(true)}if !AppCommand::controlled_retirement_supported(){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"command has no complete controlled field graph").into())}
            let bytes=size_of::<Option<AppCommand>>()+size_of::<Option<ControlledRetirement<AppCommand>>>();let d=semio_framework_value::RetirementDemand{copy_bytes:bytes,depth:1,..Default::default()};if cx.fuel_remaining()<1||!retained_clone_grant_admits(grant,d){return Ok(false)}
            cx.consume_retained(P{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);let Some(command)=self.command.take()else{return Ok(true)};self.original_close=Some(ControlledRetirement::retain(command));return Ok(false)
        }
        let Some(owner)=self.original_close.as_mut()else{return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"command retirement lost its retained cursor").into())};
        if owner.terminal_is_empty(){let bytes=size_of::<Option<ControlledRetirement<AppCommand>>>();let d=semio_framework_value::RetirementDemand{copy_bytes:bytes,depth:1,..Default::default()};if cx.fuel_remaining()<1||!retained_clone_grant_admits(grant,d){return Ok(false)}cx.consume_retained(P{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);self.original_close.take();return Ok(true)}
        let demand=owner.next_demand(grant.maximum_copy_bytes)?;let work=owner.next_computational_work(grant.maximum_copy_bytes)?;if cx.fuel_remaining()<work||!retained_clone_grant_admits(grant,demand){return Ok(false)}
        super::original_cursor::close_controlled(owner,cx)?;Ok(false)
    }

    pub fn take_for_dispatch(&mut self) -> Option<AppCommand> {
        self.command.take()
    }

    pub fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
        if self.original_close.is_some(){return (false,0,0)}
        let Some(command) = self.command.as_mut() else {
            return (true, 0, 0);
        };
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
            (2, AppCommand::SubmitMediaExport { checkpoint_id, .. }) => checkpoint_id.take().map(String::into_bytes),
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
                    (2, AppCommand::SubmitMediaExport { checkpoint_id, .. }) => *checkpoint_id=Some(String::from_utf8(field).expect("decoded checkpoint retains valid UTF-8")),
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
        self.command.is_none()&&self.original_close.is_none()&&self.archive_close.is_none()
    }
}

/// 🧭️ Which page index an ingress cursor is expected to carry — see `validate_cursor`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum CursorShape {
    Page,
    Terminal,
    Fault,
}
//#endregion 🔖️PagedAppCommandDecode

impl std::fmt::Debug for PagedAppCommandDecodeCursor{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{f.debug_struct("PagedAppCommandDecodeCursor").field("reader",&self.reader).field("state",&self.state).field("original",&self.original).field("closing",&self.original_close.is_some()).finish()}}
