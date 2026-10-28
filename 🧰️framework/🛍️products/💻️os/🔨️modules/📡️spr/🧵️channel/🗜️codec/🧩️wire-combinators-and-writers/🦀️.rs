//! 🧩️ Wire combinators and writers of the channel codec: the malformed error, the option and vector readers and writers, the command page writer and the media export handle codecs. See [`super`].

use super::*;

//#region 🔖️Codec
// Hand-rolled binary frame encode/decode: `tag: u8 | fields...` — see the module-level docstring.
// `protocol::wire::🔖️WireCodec` supplies the primitives; this crate adds only the option/vec
// combinators the frame shapes need plus the tag-dispatch match arms below.

// 🚫️async: R9 pure accessor — most call sites are inside `.ok_or_else`'s sync closure
// (Option::ok_or_else requires sync FnOnce); no suspension point exists in the body either.
pub(in super::super) fn malformed(what: &'static str, offset: u64, detail: &str) -> protocol::ProtocolError {
    protocol::ProtocolError::Malformed { what, offset, detail: detail.to_string() }
}

//#region 🔖️Combinators
pub(super) async fn write_opt_u64(out: &mut Vec<u8>, value: &Option<u64>) {
    protocol::write_bool(out, value.is_some());
    if let Some(v) = value {
        protocol::write_varint_u64(out, *v);
    }
}

pub(super) async fn read_opt_u64(bytes: &[u8], pos: &mut usize) -> Result<Option<u64>, protocol::ProtocolError> {
    if protocol::read_bool(bytes, pos)? {
        Ok(Some(protocol::read_varint_u64(bytes, pos)?))
    } else {
        Ok(None)
    }
}

pub(super) fn read_invocation_result_pack(bytes: &[u8], pos: &mut usize) -> Result<Vec<u8>, protocol::ProtocolError> {
    let start = *pos;
    let length = usize::try_from(protocol::read_varint_u64(bytes, pos)?).map_err(|_| malformed("channel invocation result pack", start as u64, "packed mutations or inverse group length does not fit this host"))?;
    if length > INVOCATION_RESULT_PACK_MAXIMUM_BYTES {
        return Err(malformed("channel invocation result pack", start as u64, "packed mutations or inverse group exceed command transport authority"));
    }
    let end = pos.checked_add(length).ok_or_else(|| malformed("channel invocation result pack", start as u64, "packed mutations or inverse group length overflow"))?;
    let value = bytes.get(*pos..end).ok_or_else(|| malformed("channel invocation result pack", start as u64, "packed mutations or inverse group are truncated"))?.to_vec();
    *pos = end;
    Ok(value)
}

/// 🎞️ `presence u8 | byte` — an `Option<u8>` (`AppCommand::Presence.own_color`), the same
/// presence-byte convention as {@link write_opt_u64} above.
pub(super) async fn write_opt_u8(out: &mut Vec<u8>, value: &Option<u8>) {
    protocol::write_bool(out, value.is_some());
    if let Some(v) = value {
        out.push(*v);
    }
}

pub(super) async fn read_opt_u8(bytes: &[u8], pos: &mut usize) -> Result<Option<u8>, protocol::ProtocolError> {
    if protocol::read_bool(bytes, pos)? {
        let byte = *bytes.get(*pos).ok_or_else(|| malformed("channel app-command opt-u8", *pos as u64, "truncated"))?;
        *pos += 1;
        Ok(Some(byte))
    } else {
        Ok(None)
    }
}

pub(super) async fn write_vec_bytes(out: &mut Vec<u8>, values: &[Vec<u8>]) {
    protocol::write_varint_u64(out, values.len() as u64);
    for value in values {
        protocol::write_bytes(out, value);
    }
}

pub(super) async fn read_vec_bytes(bytes: &[u8], pos: &mut usize) -> Result<Vec<Vec<u8>>, protocol::ProtocolError> {
    let count = protocol::read_varint_u64(bytes, pos)?;
    let mut out = Vec::with_capacity(count as usize);
    for _ in 0..count {
        out.push(protocol::read_bytes(bytes, pos)?);
    }
    Ok(out)
}

pub(super) async fn write_presence_roster(out: &mut Vec<u8>, roster: &PresenceRosterWire) {
    protocol::write_varint_u64(out, roster.len() as u64);
    for entry in roster.iter() {
        protocol::write_bytes(out, entry);
    }
}

pub(super) async fn write_vec_envelope(out: &mut Vec<u8>, values: &[protocol::causal::MutationEnvelope]) {
    protocol::write_varint_u64(out, values.len() as u64);
    for value in values {
        protocol::io::binary::causal::encode_envelope(value, out);
    }
}

pub(super) async fn read_vec_envelope(bytes: &[u8], pos: &mut usize) -> Result<Vec<protocol::causal::MutationEnvelope>, protocol::ProtocolError> {
    let count = protocol::read_varint_u64(bytes, pos)?;
    let mut out = Vec::with_capacity(count as usize);
    for _ in 0..count {
        out.push(protocol::io::binary::causal::decode_envelope(bytes, pos)?);
    }
    Ok(out)
}
//#endregion 🔖️Combinators

/// 📤️ Encodes one `AppCommand`: `tag u8 | fields`.

pub(super) struct CommandPageWriter {
    pub(super) pages: CommandPageSet,
    pub(super) current: [u8; COMMAND_PAGE_MAXIMUM_BYTES],
    pub(super) current_len: usize,
    pub(in super::super) bytes: usize,
}

impl CommandPageWriter {
    /// ✍️ The generic encoder is the ONE authority that cannot declare its page count up front — it
    /// discovers it while writing — so it declares the whole 64-page ceiling. It runs on a host, not
    /// on the guest's fixed linear memory: the guest never encodes, it only admits pages.
    pub(in super::super) fn try_new() -> Result<Self, semio_framework_diagnostic::Fault> {
        Ok(Self { pages: CommandPageSet::try_new(COMMAND_MAXIMUM_PAGES)?, current: [0; COMMAND_PAGE_MAXIMUM_BYTES], current_len: 0, bytes: 0 })
    }

    pub(super) fn flush(&mut self) -> Result<(), semio_framework_diagnostic::Fault> {
        let current = std::mem::replace(&mut self.current, [0; COMMAND_PAGE_MAXIMUM_BYTES]);
        let page = FixedCommandPage::try_from_array(current, self.current_len as u32)?;
        self.current_len = 0;
        self.pages.try_push(page).map_err(|(fault, _page)| fault)
    }

    pub(super) fn write(&mut self, mut bytes: &[u8]) -> Result<(), semio_framework_diagnostic::Fault> {
        while !bytes.is_empty() {
            if self.current_len == COMMAND_PAGE_MAXIMUM_BYTES {
                self.flush()?;
            }
            let take = bytes.len().min(COMMAND_PAGE_MAXIMUM_BYTES - self.current_len);
            self.current[self.current_len..self.current_len + take].copy_from_slice(&bytes[..take]);
            self.current_len += take;
            bytes = &bytes[take..];
            self.bytes += take;
        }
        Ok(())
    }

    pub(super) fn byte(&mut self, byte: u8) -> Result<(), semio_framework_diagnostic::Fault> {
        self.write(&[byte])
    }

    pub(in super::super) fn varint(&mut self, mut value: u64) -> Result<(), semio_framework_diagnostic::Fault> {
        loop {
            let mut byte = (value & 0x7f) as u8;
            value >>= 7;
            if value != 0 {
                byte |= 0x80;
            }
            self.byte(byte)?;
            if value == 0 {
                return Ok(());
            }
        }
    }

    pub(in super::super) fn bytes(&mut self,source:&(impl semio_framework_value::ByteReadSource+?Sized))->Result<(),semio_framework_diagnostic::Fault>{
        self.varint(source.byte_len()as u64)?;let mut at=0;let mut page=[0;256];while at<source.byte_len(){let count=source.read_bytes(at,&mut page);assert!(count>0&&count<=source.byte_len()-at,"original byte source must preserve its declared length");self.write(&page[..count])?;at+=count;}Ok(())
    }

    pub(in super::super) fn string(&mut self, value: &str) -> Result<(), semio_framework_diagnostic::Fault> {
        self.bytes(value.as_bytes())
    }

    pub(super) fn envelope(&mut self, value: &protocol::causal::MutationEnvelope) -> Result<(), semio_framework_diagnostic::Fault> {
        self.string(&value.mutation_id.0)?;
        self.string(&value.document_id.0)?;
        self.string(&value.actor.0)?;
        self.varint(value.dependencies.len() as u64)?;
        for dependency in &value.dependencies {
            self.string(&dependency.0)?;
        }
        match &value.observed {
            Some(observed) => {
                self.varint(1)?;
                self.string(&observed.0)?;
            }
            None => self.varint(0)?,
        }
        self.varint(value.target.len() as u64)?;
        for segment in &value.target {
            self.string(segment)?;
        }
        self.string(&value.diff.schema.0)?;
        self.bytes(&value.diff.payload)?;
        self.string(&value.inverse.schema.0)?;
        self.bytes(&value.inverse.payload)?;
        self.varint(value.timestamp.actor)?;
        self.varint(value.timestamp.physical_ms)?;
        self.varint(value.timestamp.logical)?;
        match &value.transaction {
            Some(transaction) => {
                self.varint(1)?;
                self.string(&transaction.id)?;
                self.string(&transaction.tool)
            }
            None => self.varint(0),
        }
    }

    pub(in super::super) fn finish(mut self) -> Result<PagedCommand, semio_framework_diagnostic::Fault> {
        if self.current_len != 0 {
            self.flush()?;
        }
        PagedCommand::try_from_pages(self.pages).map_err(|(fault, _pages)| fault)
    }
}

pub(super) fn write_media_export_handle_paged(out: &mut CommandPageWriter, handle: &MediaExportHandleWire) -> Result<(), semio_framework_diagnostic::Fault> {
    out.varint(u64::from(handle.app_instance_id))?;
    out.string(&handle.parent_document_id)?;
    out.varint(handle.operation_id)?;
    out.varint(handle.base_revision)?;
    out.varint(handle.generation)
}

pub(super) fn write_media_export_handle(out: &mut Vec<u8>, handle: &MediaExportHandleWire) {
    protocol::write_varint_u64(out, u64::from(handle.app_instance_id));
    protocol::write_str(out, &handle.parent_document_id);
    protocol::write_varint_u64(out, handle.operation_id);
    protocol::write_varint_u64(out, handle.base_revision);
    protocol::write_varint_u64(out, handle.generation);
}

pub(super) fn read_media_export_handle(bytes: &[u8], pos: &mut usize) -> Result<MediaExportHandleWire, protocol::ProtocolError> {
    let app_instance_id = u32::try_from(protocol::read_varint_u64(bytes, pos)?).map_err(|_| malformed("media export app instance", *pos as u64, "app instance exceeds u32"))?;
    Ok(MediaExportHandleWire {
        app_instance_id,
        parent_document_id: protocol::read_str(bytes, pos)?,
        operation_id: protocol::read_varint_u64(bytes, pos)?,
        base_revision: protocol::read_varint_u64(bytes, pos)?,
        generation: protocol::read_varint_u64(bytes, pos)?,
    })
}
//#endregion 🔖️Codec
