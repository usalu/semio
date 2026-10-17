//! 🛣️ Route and media decode of the paged command decoders: the route field plan and the paged route field and media export handle decodes. See [`super`].

use super::*;

//#region 🔖️PagedAppCommandDecode
/// 🧭️ One route-specific command's flat field plan, in `encode_app_command`'s own write order:
/// how many length-prefixed fields precede the `prepared_ops` roster, whether the roster is written
/// at all, and how many fields follow it. The decoder below and the encoder above are twins — this
/// table is the ONE place their field counts are stated, so a new field on either side is a
/// one-line change on both.
pub(super) const fn route_field_plan(tag: u8) -> Option<(usize, bool, usize)> {
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
pub(super) struct PagedRouteFieldsDecode {
    pub(super) tag: u8,
    pub(super) leading: usize,
    pub(super) has_ops: bool,
    pub(super) trailing: usize,
    pub(super) fields: Vec<Vec<u8>>,
    pub(in super::super) ops: Vec<Vec<u8>>,
    pub(super) ops_remaining: Option<usize>,
}

impl PagedRouteFieldsDecode {
    pub(in super::super) fn new(tag: u8, plan: (usize, bool, usize)) -> Self {
        let (leading, has_ops, trailing) = plan;
        Self { tag, leading, has_ops, trailing, fields: Vec::with_capacity(leading + trailing), ops: Vec::new(), ops_remaining: None }
    }

    pub(in super::super) fn text(fields: &mut std::vec::IntoIter<Vec<u8>>) -> Result<String, Vec<u8>> {
        let bytes = fields.next().expect("retained route field");
        String::from_utf8(bytes).map_err(std::string::FromUtf8Error::into_bytes)
    }

    pub(in super::super) fn step(&mut self, seq: u64, reader: &mut PagedCommandReader) -> Result<Option<AppCommand>, semio_framework_diagnostic::Fault> {
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

    pub(in super::super) fn finish(&mut self, seq: u64) -> Result<AppCommand, semio_framework_diagnostic::Fault> {
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

    pub(in super::super) fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize) {
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
pub(super) struct PagedMediaExportHandleDecode {
    pub(super) stage: u8,
    pub(in super::super) app_instance_id: u32,
    pub(in super::super) parent_document_id: Option<String>,
    pub(in super::super) operation_id: u64,
    pub(in super::super) base_revision: u64,
    pub(super) rejected: Option<Vec<u8>>,
}

impl PagedMediaExportHandleDecode {
    pub(in super::super) fn step(&mut self, reader: &mut PagedCommandReader) -> Result<Option<MediaExportHandleWire>, semio_framework_diagnostic::Fault> {
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

    pub(in super::super) fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize) {
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
//#endregion 🔖️PagedAppCommandDecode
