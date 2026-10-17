//! 🪪️ Normalize kind cursor of the codec ABI: the structural cursor that normalizes one stdio format kind one byte at a time. See [`super`].

use super::*;

//#region ⏳️RetainedOperation
pub(super) struct NormalizeKindStructuralCursor {
    pub(super) declared_input_bytes: usize,
    pub(super) kind: [u8; OS_HOST_CODEC_MAX_KIND_BYTES],
    pub(super) kind_cursor: usize,
    pub(super) utf8: Utf8Cursor,
    pub(super) output: Option<Vec<u8>>,
    pub(super) failure: Option<OsHostCodecFailure>,
}

impl NormalizeKindStructuralCursor {
    pub(super) fn new(declared_input_bytes: usize) -> Self {
        let failure = (declared_input_bytes == 0).then(|| OsHostCodecFailure::fixed(OsHostCodecErrorCode::MalformedRequest, "missing stdio format kind"));
        Self { declared_input_bytes, kind: [0; OS_HOST_CODEC_MAX_KIND_BYTES], kind_cursor: 0, utf8: Utf8Cursor::new(), output: None, failure }
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
        if self.kind_cursor >= self.declared_input_bytes || self.kind_cursor >= self.kind.len() {
            self.fail(OsHostCodecErrorCode::InputLimit, "stdio format kind exceeds input limit");
            return;
        }
        self.kind[self.kind_cursor] = byte;
        self.kind_cursor += 1;
        self.utf8.feed(byte);
        if self.utf8.invalid {
            self.fail(OsHostCodecErrorCode::InvalidUtf8, "stdio format kind is not UTF-8");
        } else if self.kind_cursor == self.declared_input_bytes {
            if self.utf8.complete() {
                self.resolve(resolver);
            } else {
                self.fail(OsHostCodecErrorCode::InvalidUtf8, "stdio format kind is not UTF-8");
            }
        }
    }

    fn resolve<R: OsHostFormatResolver>(&mut self, resolver: &mut R) {
        let kind = match std::str::from_utf8(&self.kind[..self.kind_cursor]) {
            Ok(kind) => kind,
            Err(_) => {
                self.fail(OsHostCodecErrorCode::InvalidUtf8, "stdio format kind is not UTF-8");
                return;
            }
        };
        match resolver.resolve_format(kind) {
            Ok(Some(row)) if row.short_id.len().checked_add(6).is_some_and(|bytes| bytes <= OS_HOST_CODEC_MAX_OUTPUT_BYTES) => self.output = Some(row.short_id.into_bytes()),
            Ok(Some(_)) => self.fail(OsHostCodecErrorCode::OutputLimit, "OS host codec reply exceeds output limit"),
            Ok(None) => self.fail(OsHostCodecErrorCode::UnknownKind, "unknown stdio format kind"),
            Err(failure) => self.failure = Some(failure),
        }
    }

    pub(super) fn finish(&mut self) -> Result<Vec<u8>, OsHostCodecFailure> {
        if self.kind_cursor != self.declared_input_bytes {
            self.fail(OsHostCodecErrorCode::MalformedRequest, "truncated stdio format kind");
        } else if !self.utf8.complete() {
            self.fail(OsHostCodecErrorCode::InvalidUtf8, "stdio format kind is not UTF-8");
        }
        if let Some(failure) = self.failure.take() {
            return Err(failure);
        }
        self.output.take().ok_or_else(|| OsHostCodecFailure::fixed(OsHostCodecErrorCode::InvalidState, "normalized stdio format kind output is unavailable"))
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
        true
    }

    pub(super) fn terminal_is_empty(&self) -> bool {
        self.output.as_ref().is_none_or(Vec::is_empty) && self.kind_cursor == 0
    }
}
//#endregion ⏳️RetainedOperation
