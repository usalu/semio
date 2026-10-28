//! 🧭️ Command cursor of the paged command decoders: the resumable paged AppCommand decode cursor implementation. See [`super`].

use super::*;

//#region 🔖️PagedAppCommandDecode
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
            PagedAppCommandDecodeState::MediaExportSubmitRevision { seq, port, expected_parent_document_id } => {let expected_base_revision=match self.reader.read_varint(){Ok(value)=>value,Err(fault)=>{self.state=PagedAppCommandDecodeState::MediaExportSubmitRevision{seq,port,expected_parent_document_id};return Err(fault)}};self.state=PagedAppCommandDecodeState::MediaExportSubmitSelection{seq,port,expected_parent_document_id,expected_base_revision};None}
            PagedAppCommandDecodeState::MediaExportSubmitSelection { seq,mut port,mut expected_parent_document_id,expected_base_revision } => {let selection=match self.reader.read_byte(){Ok(value)=>value,Err(fault)=>{self.state=PagedAppCommandDecodeState::MediaExportSubmitSelection{seq,port,expected_parent_document_id,expected_base_revision};return Err(fault)}};match selection{0=>Some(AppCommand::SubmitMediaExport{seq,port:port.take().unwrap(),expected_parent_document_id:expected_parent_document_id.take().unwrap(),expected_base_revision,checkpoint_id:None}),1=>{self.state=PagedAppCommandDecodeState::MediaExportSubmitCheckpoint{seq,port,expected_parent_document_id,expected_base_revision};None},_=>{self.state=PagedAppCommandDecodeState::MediaExportSubmitSelection{seq,port,expected_parent_document_id,expected_base_revision};return Err(semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework,semio_framework_diagnostic::FaultCode::new("plugin.command-checkpoint-tag"),"checkpoint selection tag must be zero or one"))}}}
            PagedAppCommandDecodeState::MediaExportSubmitCheckpoint { seq,mut port,mut expected_parent_document_id,expected_base_revision } => {
                let bytes=match self.reader.read_bounded_bytes(2048){Ok(bytes)=>bytes,Err(fault)=>{self.state=PagedAppCommandDecodeState::MediaExportSubmitCheckpoint{seq,port,expected_parent_document_id,expected_base_revision};return Err(fault)}};
                let checkpoint_id=match String::from_utf8(bytes){Ok(value)=>value,Err(error)=>{self.state=PagedAppCommandDecodeState::RejectedFields{fields:vec![port.take().unwrap().into_bytes(),expected_parent_document_id.take().unwrap().into_bytes(),error.into_bytes()]};return Err(semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework,semio_framework_diagnostic::FaultCode::new("plugin.command-field-utf8"),"checkpoint selection is not valid UTF-8"))}};
                if checkpoint_id.is_empty()||checkpoint_id.chars().count()>512{self.state=PagedAppCommandDecodeState::RejectedFields{fields:vec![port.take().unwrap().into_bytes(),expected_parent_document_id.take().unwrap().into_bytes(),checkpoint_id.into_bytes()]};return Err(semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework,semio_framework_diagnostic::FaultCode::new("plugin.command-checkpoint-identity"),"checkpoint identity exceeds its bound"))}
                Some(AppCommand::SubmitMediaExport{seq,port:port.take().unwrap(),expected_parent_document_id:expected_parent_document_id.take().unwrap(),expected_base_revision,checkpoint_id:Some(checkpoint_id)})
            }
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
                    AppCommand::SubmitMediaExport { port, expected_parent_document_id, checkpoint_id,.. } => {let mut fields=vec![port.into_bytes(),expected_parent_document_id.into_bytes()];if let Some(id)=checkpoint_id{fields.push(id.into_bytes());}fields},
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
            PagedAppCommandDecodeState::MediaExportSubmitRevision { port, expected_parent_document_id, .. } | PagedAppCommandDecodeState::MediaExportSubmitSelection { port, expected_parent_document_id, .. } | PagedAppCommandDecodeState::MediaExportSubmitCheckpoint { port, expected_parent_document_id, .. } => Some(if port.is_some() { port } else { expected_parent_document_id }),
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
