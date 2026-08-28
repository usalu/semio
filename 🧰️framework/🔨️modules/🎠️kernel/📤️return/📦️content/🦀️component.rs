//#region 📦️RetainedContentFraming
pub const RETURN_CONTENT_HEADER_MAXIMUM_BYTES: usize = 11;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ReturnContentWriteProgress { pub written_bytes: usize, pub complete: bool }

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ReturnContentReadProgress { pub consumed_bytes: usize, pub complete: bool }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ReturnContentReadError { pub consumed_bytes: usize, pub reason: &'static str }

pub struct ReturnContentHeader { bytes: [u8; RETURN_CONTENT_HEADER_MAXIMUM_BYTES], length: u8, offset: u8 }

impl ReturnContentHeader {
    /// 🏷️ Captures one fixed record prefix; the declared body extent is never allocation permission.
    pub fn new(tag: u8, mut body_length: u64) -> Result<Self, &'static str> {
        if tag > 9 { return Err("return-content.header-tag"); }
        let mut bytes = [0; RETURN_CONTENT_HEADER_MAXIMUM_BYTES];
        bytes[0] = tag;
        let mut length = 1;
        loop {
            let byte = (body_length & 127) as u8;
            body_length >>= 7;
            bytes[length] = byte | if body_length == 0 { 0 } else { 128 };
            length += 1;
            if body_length == 0 { break; }
        }
        Ok(Self { bytes, length: length as u8, offset: 0 })
    }

    pub fn is_complete(&self) -> bool { self.offset == self.length }

    /// 📤️ Advances only within the caller's item/byte grant, leaving all unused destination bytes unchanged.
    pub fn write(&mut self, output: &mut [u8], maximum_items: usize, maximum_bytes: usize) -> ReturnContentWriteProgress {
        if maximum_items == 0 || maximum_bytes == 0 { return ReturnContentWriteProgress { written_bytes: 0, complete: self.is_complete() }; }
        let written_bytes = usize::from(self.length - self.offset).min(output.len()).min(maximum_bytes);
        let start = usize::from(self.offset);
        output[..written_bytes].copy_from_slice(&self.bytes[start..start + written_bytes]);
        self.offset += written_bytes as u8;
        ReturnContentWriteProgress { written_bytes, complete: self.is_complete() }
    }
}

#[derive(Clone, Copy)]
enum ReadState { Tag, Length, Complete, Fault(&'static str) }

pub struct ReturnContentHeaderReader { tag: u8, length: u64, index: u8, state: ReadState }

impl Default for ReturnContentHeaderReader {
    fn default() -> Self { Self::new() }
}

impl ReturnContentHeaderReader {
    pub fn new() -> Self { Self { tag: 0, length: 0, index: 0, state: ReadState::Tag } }

    pub fn value(&self) -> Option<(u8, u64)> {
        matches!(self.state, ReadState::Complete).then_some((self.tag, self.length))
    }

    /// 🧩️ Owns each consumed prefix in fixed scalar state; no borrowed page is kept across calls.
    pub fn consume(&mut self, input: &[u8], maximum_items: usize, maximum_bytes: usize) -> Result<ReturnContentReadProgress, ReturnContentReadError> {
        if let ReadState::Fault(reason) = self.state { return Err(ReturnContentReadError { consumed_bytes: 0, reason }); }
        if maximum_items == 0 || maximum_bytes == 0 || self.value().is_some() { return Ok(ReturnContentReadProgress { consumed_bytes: 0, complete: self.value().is_some() }); }
        let limit = input.len().min(maximum_bytes).min(RETURN_CONTENT_HEADER_MAXIMUM_BYTES);
        for (index, byte) in input[..limit].iter().copied().enumerate() {
            if let Err(reason) = self.consume_byte(byte) {
                self.state = ReadState::Fault(reason);
                return Err(ReturnContentReadError { consumed_bytes: index + 1, reason });
            }
            if self.value().is_some() { return Ok(ReturnContentReadProgress { consumed_bytes: index + 1, complete: true }); }
        }
        Ok(ReturnContentReadProgress { consumed_bytes: limit, complete: false })
    }

    /// 🏁️ Distinguishes a complete prefix from truncated terminal input without inventing an empty body.
    pub fn finish(&self) -> Result<(u8, u64), &'static str> {
        match self.state {
            ReadState::Complete => Ok((self.tag, self.length)),
            ReadState::Fault(reason) => Err(reason),
            ReadState::Tag | ReadState::Length => Err("return-content.header-truncated"),
        }
    }

    fn consume_byte(&mut self, byte: u8) -> Result<(), &'static str> {
        match self.state {
            ReadState::Tag => {
                if byte > 9 { return Err("return-content.header-tag"); }
                self.tag = byte;
                self.state = ReadState::Length;
            }
            ReadState::Length => {
                if self.index == 9 && byte & 126 != 0 { return Err("return-content.header-overflow"); }
                self.length |= u64::from(byte & 127) << (u32::from(self.index) * 7);
                if byte & 128 == 0 {
                    if self.index != 0 && byte == 0 { return Err("return-content.header-noncanonical"); }
                    self.state = ReadState::Complete;
                } else {
                    if self.index == 9 { return Err("return-content.header-overlong"); }
                    self.index += 1;
                }
            }
            ReadState::Complete => return Err("return-content.header-already-complete"),
            ReadState::Fault(reason) => return Err(reason),
        }
        Ok(())
    }
}
//#endregion 📦️RetainedContentFraming
