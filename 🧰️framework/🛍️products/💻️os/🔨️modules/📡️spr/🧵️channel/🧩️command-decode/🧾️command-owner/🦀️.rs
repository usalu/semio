//! 🧾️ Command owner of the paged command decoders: the decode state, the cursor and owner records, the owner implementation and the cursor shape. See [`super`].

use super::*;

//#region 🔖️PagedAppCommandDecode
#[derive(Debug)]
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
    MediaExportHandle { seq: u64, kind: u8, decode: PagedMediaExportHandleDecode },
    RejectedFields { fields: Vec<Vec<u8>> },
    Terminal,
    Faulted,
}

#[derive(Debug)]
pub struct PagedAppCommandDecodeCursor {
    pub(super) reader: PagedCommandReader,
    pub(in super::super) state: PagedAppCommandDecodeState,
}

pub struct DecodedAppCommandOwner {
    pub(in super::super) command: Option<AppCommand>,
    pub(super) close_stage: u8,
    pub(super) archive_close: Option<PagedDocumentArchiveDecode>,
    pub(super) text_close:Option<semio_framework_value::retirement::controlled::ControlledRetirement<String>>,
    pub(super) bytes_close:Option<semio_framework_value::retirement::controlled::ControlledRetirement<Vec<u8>>>,
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
pub(super) enum CursorShape {
    Page,
    Terminal,
    Fault,
}
//#endregion 🔖️PagedAppCommandDecode
