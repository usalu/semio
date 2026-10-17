//! 🔎️ Filter kinds cursor of the codec ABI: the structural cursor that filters accepted media kinds one byte at a time. See [`super`].

use super::*;

//#region ⏳️RetainedOperation
pub(super) struct FilterKindsStructuralCursor {
    header: [u8; 3],
    header_cursor: usize,
    count: usize,
    pub(super) index: usize,
    length: [u8; 2],
    length_cursor: usize,
    expected_kind_bytes: usize,
    pub(super) kind: [u8; OS_HOST_CODEC_MAX_KIND_BYTES],
    pub(super) kind_cursor: usize,
    pub(super) utf8: Utf8Cursor,
    pub(super) output: Option<Vec<u8>>,
    pub(super) complete: bool,
    pub(super) failure: Option<OsHostCodecFailure>,
}

impl FilterKindsStructuralCursor {
    pub(super) fn new() -> Self {
        Self {
            header: [0; 3],
            header_cursor: 0,
            count: 0,
            index: 0,
            length: [0; 2],
            length_cursor: 0,
            expected_kind_bytes: 0,
            kind: [0; OS_HOST_CODEC_MAX_KIND_BYTES],
            kind_cursor: 0,
            utf8: Utf8Cursor::new(),
            output: Some(Vec::new()),
            complete: false,
            failure: None,
        }
    }

    pub(super) fn fail(&mut self, code: OsHostCodecErrorCode, message: &'static str) {
        if self.failure.is_none() {
            self.failure = Some(OsHostCodecFailure::fixed(code, message));
        }
    }

    pub(super) fn feed<R: OsHostFormatResolver>(&mut self, byte: u8, resolver: &mut R) {
        if self.failure.is_some() {
            return;
        }
        if self.complete {
            self.fail(OsHostCodecErrorCode::MalformedRequest, "trailing stdio format kind array bytes");
            return;
        }
        if self.header_cursor < self.header.len() {
            self.header[self.header_cursor] = byte;
            self.header_cursor += 1;
            if self.header_cursor == self.header.len() {
                self.finish_header();
            }
            return;
        }
        if self.length_cursor < self.length.len() {
            self.length[self.length_cursor] = byte;
            self.length_cursor += 1;
            if self.length_cursor == self.length.len() {
                self.expected_kind_bytes = u16::from_le_bytes(self.length) as usize;
                if self.expected_kind_bytes > OS_HOST_CODEC_MAX_KIND_BYTES {
                    self.fail(OsHostCodecErrorCode::InputLimit, "stdio format kind exceeds input limit");
                } else if self.expected_kind_bytes == 0 {
                    self.finish_kind(resolver);
                }
            }
            return;
        }
        if self.kind_cursor >= self.expected_kind_bytes || self.kind_cursor >= self.kind.len() {
            self.fail(OsHostCodecErrorCode::MalformedRequest, "malformed stdio format kind array");
            return;
        }
        self.kind[self.kind_cursor] = byte;
        self.kind_cursor += 1;
        self.utf8.feed(byte);
        if self.utf8.invalid {
            self.fail(OsHostCodecErrorCode::InvalidUtf8, "stdio format kind is not UTF-8");
        } else if self.kind_cursor == self.expected_kind_bytes {
            if self.utf8.complete() {
                self.finish_kind(resolver);
            } else {
                self.fail(OsHostCodecErrorCode::InvalidUtf8, "stdio format kind is not UTF-8");
            }
        }
    }

    fn finish_header(&mut self) {
        if self.header[0] != 1 {
            self.fail(OsHostCodecErrorCode::MalformedRequest, "invalid stdio format kind array version");
            return;
        }
        self.count = u16::from_le_bytes([self.header[1], self.header[2]]) as usize;
        if self.count > OS_HOST_CODEC_MAX_KIND_COUNT {
            self.fail(OsHostCodecErrorCode::InputLimit, "stdio format kind array exceeds item limit");
        } else if self.count == 0 {
            self.complete = true;
        }
    }

    fn finish_kind<R: OsHostFormatResolver>(&mut self, resolver: &mut R) {
        let kind = match std::str::from_utf8(&self.kind[..self.kind_cursor]) {
            Ok(kind) => kind,
            Err(_) => {
                self.fail(OsHostCodecErrorCode::InvalidUtf8, "stdio format kind is not UTF-8");
                return;
            }
        };
        let row = match resolver.resolve_format(kind) {
            Ok(Some(row)) => row,
            Ok(None) => {
                self.fail(OsHostCodecErrorCode::UnknownKind, "unknown stdio format kind");
                return;
            }
            Err(failure) => {
                self.failure = Some(failure);
                return;
            }
        };
        for extension in row.extensions {
            let output = self.output.as_mut().expect("filter output is retained until seal");
            let separator = usize::from(!output.is_empty());
            let Some(output_bytes) = output.len().checked_add(separator).and_then(|bytes| bytes.checked_add(extension.len())).and_then(|bytes| bytes.checked_add(6)) else {
                self.fail(OsHostCodecErrorCode::OutputLimit, "OS host codec reply exceeds output limit");
                return;
            };
            if output_bytes > OS_HOST_CODEC_MAX_OUTPUT_BYTES {
                self.fail(OsHostCodecErrorCode::OutputLimit, "OS host codec reply exceeds output limit");
                return;
            }
            if separator != 0 {
                output.push(b',');
            }
            output.extend_from_slice(extension.as_bytes());
        }
        self.index += 1;
        self.length = [0; 2];
        self.length_cursor = 0;
        self.expected_kind_bytes = 0;
        self.kind[..self.kind_cursor].fill(0);
        self.kind_cursor = 0;
        self.utf8 = Utf8Cursor::new();
        self.complete = self.index == self.count;
    }

    pub(super) fn finish(&mut self) -> Result<Vec<u8>, OsHostCodecFailure> {
        if self.header_cursor == 0 {
            self.fail(OsHostCodecErrorCode::MissingKindArray, "missing stdio format kind array");
        } else if self.header_cursor != self.header.len() || !self.complete || self.length_cursor != 0 || self.kind_cursor != 0 {
            self.fail(OsHostCodecErrorCode::MalformedRequest, "truncated stdio format kind array");
        }
        if let Some(failure) = self.failure.take() {
            return Err(failure);
        }
        self.output.take().ok_or_else(|| OsHostCodecFailure::fixed(OsHostCodecErrorCode::InvalidState, "stdio format filter output is unavailable"))
    }

    pub(super) fn close_one(&mut self) -> bool {
        if self.output.as_mut().is_some_and(|output| output.pop().is_some()) {
            return false;
        }
        if self.kind_cursor != 0 {
            self.kind_cursor -= 1;
            self.kind[self.kind_cursor] = 0;
            return false;
        }
        if self.length_cursor != 0 {
            self.length_cursor -= 1;
            self.length[self.length_cursor] = 0;
            return false;
        }
        if self.header_cursor != 0 {
            self.header_cursor -= 1;
            self.header[self.header_cursor] = 0;
            return false;
        }
        true
    }

    pub(super) fn terminal_is_empty(&self) -> bool {
        self.output.as_ref().is_none_or(Vec::is_empty) && self.kind_cursor == 0 && self.length_cursor == 0 && self.header_cursor == 0
    }
}
//#endregion ⏳️RetainedOperation
