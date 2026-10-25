//! 🧱️ Message codec of the ABI: the canonical versioned little-endian encoder and decoder of one message. See [`super`].

use super::*;

//#region 🧱️Codec
/// 📦️ Encodes the canonical versioned, little-endian structural ledger.
pub fn encode_abi_message(message: &AbiMessage) -> Vec<u8> {
    let mut encoder = Encoder::new();
    encoder.u8(ABI_VERSION);
    match message {
        AbiMessage::Request(value) => {
            encoder.u8(1);
            encoder.u16(value.operation.get());
            encoder.u64(value.request_id.0);
            encoder.u32(value.generation);
            encoder.bytes(value.bytes.as_slice());
        }
        AbiMessage::Reply(value) => {
            encoder.u8(2);
            encoder.u64(value.request_id.0);
            encoder.u32(value.generation);
            encoder.status(&value.status);
            encoder.bytes(value.bytes.as_slice());
        }
        AbiMessage::Event(value) => {
            encoder.u8(3);
            encoder.u64(value.request_id.0);
            encoder.u32(value.generation);
            encoder.u32(value.sequence);
            encoder.u16(value.event.get());
            encoder.status(&value.status);
            encoder.bytes(value.bytes.as_slice());
        }
        AbiMessage::Page(value) => {
            encoder.u8(4);
            encoder.handle(value.handle);
            encoder.u32(value.index);
            encoder.bytes(value.bytes.as_slice());
        }
        AbiMessage::Control(value) => {
            encoder.u8(5);
            match value {
                AbiControl::Cancel { request_id, generation } => {
                    encoder.u8(1);
                    encoder.u64(request_id.0);
                    encoder.u32(*generation);
                }
                AbiControl::Close { handle } => {
                    encoder.u8(2);
                    encoder.handle(*handle);
                }
                AbiControl::Acknowledge { handle, index } => {
                    encoder.u8(3);
                    encoder.handle(*handle);
                    encoder.u32(*index);
                }
            }
        }
    }
    encoder.finish()
}

/// 🔬 Decodes exactly one structural ledger record and rejects trailing or partial fields.
pub fn decode_abi_message(bytes: &[u8]) -> Result<AbiMessage, AbiErrorCode> {
    let mut decoder = Decoder::new(bytes);
    if decoder.u8()? != ABI_VERSION {
        return Err(AbiErrorCode::MalformedTag);
    }
    let message = match decoder.u8()? {
        1 => AbiMessage::Request(AbiRequest { operation: AbiOperation::try_new(decoder.u16()?)?, request_id: AbiRequestId(decoder.u64()?), generation: decoder.u32()?, bytes: AbiBytes(decoder.bytes(ABI_MAX_BODY_BYTES)?) }),
        2 => AbiMessage::Reply(AbiReply { request_id: AbiRequestId(decoder.u64()?), generation: decoder.u32()?, status: decoder.status()?, bytes: AbiBytes(decoder.bytes(ABI_MAX_BODY_BYTES)?) }),
        3 => AbiMessage::Event(AbiEvent {
            request_id: AbiRequestId(decoder.u64()?),
            generation: decoder.u32()?,
            sequence: decoder.u32()?,
            event: AbiEventCode::try_new(decoder.u16()?)?,
            status: decoder.status()?,
            bytes: AbiBytes(decoder.bytes(ABI_MAX_BODY_BYTES)?),
        }),
        4 => {
            let handle = decoder.handle()?;
            let index = decoder.u32()?;
            if index >= ABI_MAX_PAGES_PER_TRANSFER {
                return Err(AbiErrorCode::LimitExceeded);
            }
            AbiMessage::Page(AbiPage { handle, index, bytes: AbiPageBytes(decoder.bytes(ABI_MAX_PAGE_BYTES)?) })
        }
        5 => AbiMessage::Control(match decoder.u8()? {
            1 => AbiControl::Cancel { request_id: AbiRequestId(decoder.u64()?), generation: decoder.u32()? },
            2 => AbiControl::Close { handle: decoder.handle()? },
            3 => {
                let handle = decoder.handle()?;
                let index = decoder.u32()?;
                if index >= ABI_MAX_PAGES_PER_TRANSFER {
                    return Err(AbiErrorCode::LimitExceeded);
                }
                AbiControl::Acknowledge { handle, index }
            }
            _ => return Err(AbiErrorCode::MalformedTag),
        }),
        _ => return Err(AbiErrorCode::MalformedTag),
    };
    if decoder.remaining() != 0 {
        return Err(AbiErrorCode::MalformedLength);
    }
    Ok(message)
}

struct Encoder {
    pub(super) bytes: Vec<u8>,
}

impl Encoder {
    pub(super) fn new() -> Self {
        Self { bytes: Vec::new() }
    }

    fn finish(self) -> Vec<u8> {
        self.bytes
    }

    pub(super) fn u8(&mut self, value: u8) {
        self.bytes.push(value);
    }

    pub(super) fn u16(&mut self, value: u16) {
        self.bytes.extend_from_slice(&value.to_le_bytes());
    }

    pub(super) fn u32(&mut self, value: u32) {
        self.bytes.extend_from_slice(&value.to_le_bytes());
    }

    pub(super) fn u64(&mut self, value: u64) {
        self.bytes.extend_from_slice(&value.to_le_bytes());
    }

    pub(super) fn bytes(&mut self, bytes: &[u8]) {
        self.u32(bytes.len() as u32);
        self.bytes.extend_from_slice(bytes);
    }

    pub(super) fn handle(&mut self, handle: AbiHandle) {
        self.u32(handle.slot);
        self.u32(handle.generation);
    }

    pub(super) fn status(&mut self, status: &AbiStatus) {
        self.u16(status.code as u16);
        match &status.error {
            None => self.u8(0),
            Some(error) => {
                self.u8(1);
                self.u16(error.code as u16);
                self.u16(error.message.as_bytes().len() as u16);
                self.bytes.extend_from_slice(error.message.as_bytes());
            }
        }
    }
}

struct Decoder<'a> {
    pub(super) bytes: &'a [u8],
    pub(super) cursor: usize,
}

impl<'a> Decoder<'a> {
    pub(super) fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, cursor: 0 }
    }

    pub(super) fn remaining(&self) -> usize {
        self.bytes.len() - self.cursor
    }

    pub(super) fn take(&mut self, count: usize, incomplete: AbiErrorCode) -> Result<&'a [u8], AbiErrorCode> {
        let end = self.cursor.checked_add(count).ok_or(AbiErrorCode::MalformedLength)?;
        if end > self.bytes.len() {
            return Err(incomplete);
        }
        let bytes = &self.bytes[self.cursor..end];
        self.cursor = end;
        Ok(bytes)
    }

    pub(super) fn u8(&mut self) -> Result<u8, AbiErrorCode> {
        Ok(self.take(1, AbiErrorCode::MissingField)?[0])
    }

    pub(super) fn u16(&mut self) -> Result<u16, AbiErrorCode> {
        Ok(u16::from_le_bytes(self.take(2, AbiErrorCode::MissingField)?.try_into().expect("fixed width")))
    }

    pub(super) fn u32(&mut self) -> Result<u32, AbiErrorCode> {
        Ok(u32::from_le_bytes(self.take(4, AbiErrorCode::MissingField)?.try_into().expect("fixed width")))
    }

    pub(super) fn u64(&mut self) -> Result<u64, AbiErrorCode> {
        Ok(u64::from_le_bytes(self.take(8, AbiErrorCode::MissingField)?.try_into().expect("fixed width")))
    }

    pub(super) fn bytes(&mut self, limit: usize) -> Result<Vec<u8>, AbiErrorCode> {
        let len = self.u32()? as usize;
        if len > limit {
            return Err(AbiErrorCode::LimitExceeded);
        }
        Ok(self.take(len, AbiErrorCode::MalformedLength)?.to_vec())
    }

    pub(super) fn handle(&mut self) -> Result<AbiHandle, AbiErrorCode> {
        AbiHandle::try_new(self.u32()?, self.u32()?)
    }

    pub(super) fn status(&mut self) -> Result<AbiStatus, AbiErrorCode> {
        let code = AbiStatusCode::decode(self.u16()?)?;
        let error = match self.u8()? {
            0 => None,
            1 => {
                let code = AbiErrorCode::decode(self.u16()?)?;
                let len = self.u16()? as usize;
                if len > ABI_MAX_MESSAGE_BYTES {
                    return Err(AbiErrorCode::LimitExceeded);
                }
                let bytes = self.take(len, AbiErrorCode::MalformedLength)?.to_vec();
                Some(AbiError { code, message: AbiMessageBytes::try_new(bytes).map_err(|rejected| rejected.code)? })
            }
            _ => return Err(AbiErrorCode::MalformedTag),
        };
        Ok(AbiStatus { code, error })
    }
}
//#endregion 🧱️Codec
