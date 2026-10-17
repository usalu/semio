//! 📥️ Retained zlib decoder of the zlib codec: the step result, the inflate refusal and the decoder that resumes across steps. See [`super`].

use super::*;

//#region 🦑️DissolvedEngineCodec
//#region DeflateCodec
//#region 🧵️StreamingEncode
/// 🧵️ Result of one bounded RFC 1950 retained-codec advance.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RetainedZlibStep {
    Yield,
    Complete,
    Cancelled,
}

fn retained_inflate_refusal(error: semio_framework_deflate::DeflateError) -> semio_framework_value::ValueError {
    use semio_framework_deflate::DeflateError;
    let kind = match error {
        DeflateError::OutputLimitExceeded => semio_framework_value::ValueRefusalKind::OwnershipLimit,
        DeflateError::BadBlockType | DeflateError::BadStoredLength | DeflateError::BadHuffmanCode | DeflateError::BadDistance | DeflateError::UnexpectedEnd => semio_framework_value::ValueRefusalKind::InvalidValue,
    };
    semio_framework_value::ValueError::new(kind, format!("retained zlib decode refusal: {error:?}"))
}

/// 🌊️ Resumable RFC 1950 decoder with one raw-DEFLATE transition per fuel unit.
pub struct RetainedZlibDecoder {
    pub(super) input: Vec<u8>,
    raw_end: usize,
    pub(super) cursor: usize,
    pub(super) pending: Option<u8>,
    expected_output: usize,
    expected_adler: u32,
    pub(super) a: u32,
    pub(super) b: u32,
    pub(super) output: Vec<u8>,
    inflater: semio_framework_deflate::Inflater,
    pub(super) complete: bool,
    pub(super) closing: bool,
}

impl RetainedZlibDecoder {
    /// 🌱️ Admits the input, exact output, and RFC 1951 history ownership before decoding.
    pub fn try_new(input: Vec<u8>, expected_output: usize, maximum_owned_bytes: usize) -> Result<Self, semio_framework_value::ValueError> {
        use semio_framework_value::{ValueError, ValueRefusalKind};
        if input.len() < 6 {
            return Err(ValueError::new(ValueRefusalKind::InvalidValue, "zlib stream too short"));
        }
        let cmf = input[0];
        let flg = input[1];
        if cmf & 15 != 8 || cmf >> 4 > 7 {
            return Err(ValueError::new(ValueRefusalKind::UnsupportedOwner, "unsupported zlib method or window"));
        }
        if !(u16::from(cmf) * 256 + u16::from(flg)).is_multiple_of(31) {
            return Err(ValueError::new(ValueRefusalKind::InvalidValue, "zlib CMF/FLG check failed"));
        }
        if flg & 32 != 0 {
            return Err(ValueError::new(ValueRefusalKind::UnsupportedOwner, "retained zlib decoder has no preset dictionary authority"));
        }
        let raw_end = input.len() - 4;
        let expected_adler = u32::from_be_bytes(input[raw_end..].try_into().map_err(|_| ValueError::new(ValueRefusalKind::InvalidValue, "zlib checksum width"))?);
        let window = 1usize << ((cmf >> 4) + 8);
        let admitted = input.capacity().checked_add(expected_output).and_then(|bytes| bytes.checked_add(window)).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "retained zlib ownership overflow"))?;
        if admitted > maximum_owned_bytes {
            return Err(ValueError::new(ValueRefusalKind::OwnershipLimit, "retained zlib ownership exceeds caller limit"));
        }
        let mut output = Vec::new();
        output.try_reserve_exact(expected_output).map_err(|_| ValueError::new(ValueRefusalKind::AllocationFailed, "retained zlib output allocation failed"))?;
        let actual = input.capacity().checked_add(output.capacity()).and_then(|bytes| bytes.checked_add(window)).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "retained zlib actual ownership overflow"))?;
        if actual > maximum_owned_bytes {
            return Err(ValueError::new(ValueRefusalKind::OwnershipLimit, "retained zlib actual ownership exceeds caller limit"));
        }
        let mut inflater = semio_framework_deflate::Inflater::try_new_retained(window, window).map_err(retained_inflate_refusal)?;
        let allocation = inflater.reserve_retained_history(window).map_err(|error| ValueError::new(error.kind, error.reason))?;
        if !allocation.progressed && window != 0 {
            return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "retained zlib history admission did not progress"));
        }
        Ok(Self { input, raw_end, cursor: 2, pending: None, expected_output, expected_adler, a: 1, b: 0, output, inflater, complete: false, closing: false })
    }

    /// ➡️ Advances until the current fuel or deadline is spent or the exact stream completes.
    pub fn advance(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> Result<RetainedZlibStep, semio_framework_value::ValueError> {
        use semio_framework_deflate::InflateOutcome;
        use semio_framework_value::{ValueError, ValueRefusalKind};
        if self.closing {
            return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "retained zlib decoder is closing"));
        }
        if self.complete {
            return Ok(RetainedZlibStep::Complete);
        }
        context.set_stage("deflate:zlib-decode");
        loop {
            if context.is_cancelled() {
                return Ok(RetainedZlibStep::Cancelled);
            }
            if context.should_yield() {
                return Ok(RetainedZlibStep::Yield);
            }
            if self.pending.is_none() && self.cursor < self.raw_end {
                self.pending = Some(self.input[self.cursor]);
                self.cursor += 1;
            }
            let outcome = self.inflater.advance(&mut self.pending, self.cursor == self.raw_end).map_err(retained_inflate_refusal)?;
            context.consume_fuel(1);
            match outcome {
                InflateOutcome::NeedInput if self.pending.is_none() && self.cursor == self.raw_end => return Err(ValueError::new(ValueRefusalKind::InvalidValue, "truncated retained zlib payload")),
                InflateOutcome::NeedInput => {}
                InflateOutcome::Wrote(byte) => {
                    if self.output.len() == self.expected_output {
                        return Err(ValueError::new(ValueRefusalKind::WorkLimit, "retained zlib expansion exceeds exact output extent"));
                    }
                    self.output.push(byte);
                    self.a = (self.a + u32::from(byte)) % 65521;
                    self.b = (self.b + self.a) % 65521;
                }
                InflateOutcome::Done => {
                    let unused = self.inflater.terminal_unused_bytes().ok_or_else(|| ValueError::new(ValueRefusalKind::InvariantViolated, "retained zlib missing terminal state"))?;
                    if self.pending.is_some() || self.cursor.checked_sub(unused) != Some(self.raw_end) {
                        return Err(ValueError::new(ValueRefusalKind::InvalidValue, "trailing bytes after retained zlib terminal block"));
                    }
                    if self.output.len() != self.expected_output {
                        return Err(ValueError::new(ValueRefusalKind::InvalidValue, "retained zlib output extent differs from declared extent"));
                    }
                    if (self.b << 16) | self.a != self.expected_adler {
                        return Err(ValueError::new(ValueRefusalKind::InvalidValue, "retained zlib adler32 mismatch"));
                    }
                    self.complete = true;
                    return Ok(RetainedZlibStep::Complete);
                }
            }
        }
    }

    pub fn progress(&self) -> (usize, usize) {
        (self.output.len(), self.expected_output)
    }

    pub fn owned_bytes(&self) -> Result<usize, semio_framework_value::ValueError> {
        self.input.capacity().checked_add(self.output.capacity()).and_then(|bytes| bytes.checked_add(self.inflater.retained_allocated_bytes())).ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::OwnershipLimit, "retained zlib decoder ownership overflow"))
    }

    pub fn take_output(&mut self) -> Result<Vec<u8>, semio_framework_value::ValueError> {
        if !self.complete {
            return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "retained zlib output requested before completion"));
        }
        Ok(std::mem::take(&mut self.output))
    }

    pub fn begin_close(&mut self) {
        self.closing = true;
    }

    /// ♻️ The physical bytes the next close turn releases: the output, the input, then the inflater's history allocation.
    pub fn next_close_release_bytes(&self) -> usize {
        match next_backing_bytes(&[&self.output, &self.input]) {
            0 => self.inflater.next_retained_release_allocation_bytes().unwrap_or(0),
            bytes => bytes,
        }
    }

    /// ♻️ The flat frontier depth the next close turn needs: `1` until the decoder is terminal-empty.
    pub fn next_close_depth_demand(&self) -> usize {
        usize::from(!self.terminal_is_empty())
    }

    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        if !self.closing {
            return semio_framework_job::InteractiveJobCloseStep::Blocked;
        }
        if !deflate_close_admits(grant, self.next_close_release_bytes(), self.terminal_is_empty()) {
            return deflate_close_idle();
        }
        if let Some(released_bytes) = release_next_backing(&mut [&mut self.output, &mut self.input]) {
            return deflate_close_receipt(1, released_bytes);
        }
        match self.inflater.close_retained_step(grant.maximum_items, grant.maximum_release_bytes) {
            semio_framework_deflate::RetainedInflateCloseStep::Pending { released_items, released_bytes } => deflate_close_receipt(released_items, released_bytes),
            semio_framework_deflate::RetainedInflateCloseStep::Complete => deflate_close_complete(),
        }
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.closing && self.output.capacity() == 0 && self.input.capacity() == 0 && self.inflater.retained_terminal_is_empty()
    }
}
//#endregion 🧵️StreamingEncode
//#endregion DeflateCodec
//#endregion 🦑️DissolvedEngineCodec
