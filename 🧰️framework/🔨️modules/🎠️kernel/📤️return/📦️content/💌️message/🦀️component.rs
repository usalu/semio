//#region 💌️BorrowedMessageRecord
use super::{return_content::ReturnContentHeader, Effect, MessageEndpoint};
use semio_framework_actor::byte_page::ACTOR_BYTE_PAGE_BYTES;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ReturnMessageProgress { pub advanced_items: usize, pub written_bytes: usize, pub complete: bool }

#[derive(Clone, Copy, PartialEq, Eq)]
enum Phase { Header, EndpointPrefix, EndpointText, PayloadPrefix, Payload, Complete }

/// 💌️ Borrows one immutable message source without cloning, parsing or retiring its opaque payload.
pub struct ReturnMessageCursor<'a> {
    header: ReturnContentHeader,
    endpoint_prefix: [u8; 12],
    endpoint_prefix_len: usize,
    endpoint_text: &'a [u8],
    payload_prefix: [u8; 10],
    payload_prefix_len: usize,
    payload: &'a [u8],
    phase: Phase,
    offset: usize,
}

impl<'a> ReturnMessageCursor<'a> {
    /// 🧷️ Captures only fixed prefixes and source borrows; numeric instance validation examines at most ten bytes.
    pub fn new(effect: &'a Effect) -> Result<Self, &'static str> {
        let Effect::SendMessage { target, payload } = effect else { return Err("return-content.message-effect"); };
        let mut endpoint_prefix = [0; 12];
        let (tag, numeric, endpoint_text) = match target {
            MessageEndpoint::Shell { instance } => (0, Some(instance_number(&instance.0)?), &[][..]),
            MessageEndpoint::Backbone { uri } => (1, None, uri.as_bytes()),
            MessageEndpoint::PluginInstance { id } => (2, Some(instance_number(&id.0)?), &[][..]),
            MessageEndpoint::Extension { id } => (3, None, id.as_bytes()),
            MessageEndpoint::Topic { name } => (4, None, name.as_bytes()),
        };
        endpoint_prefix[1] = tag;
        let endpoint_value = match numeric {
            Some(value) => u64::from(value),
            None => u64::try_from(endpoint_text.len()).map_err(|_| "return-content.message-extent")?,
        };
        let endpoint_prefix_len = 2 + write_unsigned(endpoint_value, &mut endpoint_prefix[2..]);
        let payload_len = u64::try_from(payload.len()).map_err(|_| "return-content.message-extent")?;
        let mut payload_prefix = [0; 10];
        let payload_prefix_len = write_unsigned(payload_len, &mut payload_prefix);
        let body_len = (endpoint_prefix_len as u64)
            .checked_add(u64::try_from(endpoint_text.len()).map_err(|_| "return-content.message-extent")?)
            .and_then(|value| value.checked_add(payload_prefix_len as u64))
            .and_then(|value| value.checked_add(payload_len))
            .ok_or("return-content.message-extent")?;
        Ok(Self { header: ReturnContentHeader::new(5, body_len)?, endpoint_prefix, endpoint_prefix_len, endpoint_text, payload_prefix, payload_prefix_len, payload, phase: Phase::Header, offset: 0 })
    }

    /// 📤️ Advances one field phase within the byte grant; no borrowed source owner changes on completion or cancellation.
    pub fn write(&mut self, output: &mut [u8], maximum_items: usize, maximum_bytes: usize) -> ReturnMessageProgress {
        if maximum_items == 0 || maximum_bytes == 0 || output.is_empty() || self.phase == Phase::Complete {
            return ReturnMessageProgress { complete: self.phase == Phase::Complete, ..Default::default() };
        }
        let limit = output.len().min(maximum_bytes).min(ACTOR_BYTE_PAGE_BYTES);
        let (written_bytes, complete, next) = match self.phase {
            Phase::Header => {
                let step = self.header.write(&mut output[..limit], 1, limit);
                (step.written_bytes, step.complete, Phase::EndpointPrefix)
            },
            Phase::EndpointPrefix => {
                let (written, complete) = copy_piece(&self.endpoint_prefix[..self.endpoint_prefix_len], &mut self.offset, &mut output[..limit]);
                (written, complete, Phase::EndpointText)
            },
            Phase::EndpointText => {
                let (written, complete) = copy_piece(self.endpoint_text, &mut self.offset, &mut output[..limit]);
                (written, complete, Phase::PayloadPrefix)
            },
            Phase::PayloadPrefix => {
                let (written, complete) = copy_piece(&self.payload_prefix[..self.payload_prefix_len], &mut self.offset, &mut output[..limit]);
                (written, complete, Phase::Payload)
            },
            Phase::Payload => {
                let (written, complete) = copy_piece(self.payload, &mut self.offset, &mut output[..limit]);
                (written, complete, Phase::Complete)
            },
            Phase::Complete => unreachable!(),
        };
        if complete { self.phase = next; self.offset = 0; }
        ReturnMessageProgress { advanced_items: 1, written_bytes, complete: self.phase == Phase::Complete }
    }
}

fn copy_piece(source: &[u8], offset: &mut usize, output: &mut [u8]) -> (usize, bool) {
    let count = (source.len() - *offset).min(output.len());
    output[..count].copy_from_slice(&source[*offset..*offset + count]);
    *offset += count;
    (count, *offset == source.len())
}

fn instance_number(value: &str) -> Result<u32, &'static str> {
    if value.is_empty() || value.len() > 10 || (value.len() > 1 && value.as_bytes()[0] == b'0') { return Err("return-content.message-instance"); }
    let mut result = 0u32;
    for byte in value.bytes() {
        if !byte.is_ascii_digit() { return Err("return-content.message-instance"); }
        result = result.checked_mul(10).and_then(|result| result.checked_add(u32::from(byte - b'0'))).ok_or("return-content.message-instance")?;
    }
    Ok(result)
}

fn write_unsigned(mut value: u64, output: &mut [u8]) -> usize {
    let mut length = 0;
    loop {
        let byte = (value & 127) as u8;
        value >>= 7;
        output[length] = byte | if value == 0 { 0 } else { 128 };
        length += 1;
        if value == 0 { return length; }
    }
}
//#endregion 💌️BorrowedMessageRecord
