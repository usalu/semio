//! 📤️ Retained zlib encoder of the zlib codec: the encoder that resumes across steps and owns its output wallet. See [`super`].

use super::*;

//#region 🦑️DissolvedEngineCodec
//#region DeflateCodec
//#region 🧵️StreamingEncode
/// 🌊️ Resumable deterministic RFC 1950 encoder using the first-party retained raw job.
pub struct RetainedZlibEncoder {
    pub(super) job: DeflateEncodeJob,
    pub(super) literal_codes: Vec<(u32, u8)>,
    pub(super) distance_codes: Vec<(u32, u8)>,
    maximum_file_bytes: usize,
    pub(super) maximum_owned_bytes: usize,
    adler_cursor: usize,
    pub(super) a: u32,
    pub(super) b: u32,
    pub(super) raw: Vec<u8>,
    pub(super) frame: Vec<u8>,
    frame_cursor: usize,
    phase: u8,
    pub(super) complete: bool,
    pub(super) closing: bool,
}

impl RetainedZlibEncoder {
    /// 🌱️ Admits fixed-Huffman tables and proved worst-case output before work starts.
    pub fn try_new(input: Vec<u8>, maximum_file_bytes: usize, maximum_owned_bytes: usize) -> Result<Self, semio_framework_value::ValueError> {
        use semio_framework_value::{ValueError, ValueRefusalKind};
        let raw_limit = input.len().checked_mul(9).and_then(|bits| bits.checked_add(17)).map(|bits| bits / 8).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "retained zlib output capacity overflow"))?;
        let file_limit = raw_limit.checked_add(6).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "retained zlib framing extent overflow"))?;
        if file_limit > maximum_file_bytes {
            return Err(ValueError::new(ValueRefusalKind::OwnershipLimit, "retained zlib output exceeds caller file limit"));
        }
        let code_bytes = 288usize
            .checked_add(32)
            .and_then(|items| items.checked_mul(size_of::<(u32, u8)>()))
            .ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "retained zlib code ownership overflow"))?;
        let admitted = input
            .capacity()
            .checked_add(HASH_SIZE.checked_mul(size_of::<i32>()).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "retained zlib hash ownership overflow"))?)
            .and_then(|bytes| bytes.checked_add(WINDOW.checked_mul(size_of::<i32>())?))
            .and_then(|bytes| bytes.checked_add(code_bytes))
            .and_then(|bytes| bytes.checked_add(raw_limit))
            .and_then(|bytes| bytes.checked_add(file_limit))
            .ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "retained zlib encoder ownership overflow"))?;
        if admitted > maximum_owned_bytes {
            return Err(ValueError::new(ValueRefusalKind::OwnershipLimit, "retained zlib encoder ownership exceeds caller limit"));
        }
        let literal_codes = build_codes(&fixed_lit_lengths());
        let distance_codes = build_codes(&fixed_dist_lengths());
        let mut job = DeflateEncodeJob::new(input, usize::MAX);
        job.writer.out.try_reserve_exact(raw_limit).map_err(|_| ValueError::new(ValueRefusalKind::AllocationFailed, "retained zlib raw output allocation failed"))?;
        let actual = job
            .retained_owned_bytes()?
            .checked_add(literal_codes.capacity().checked_mul(size_of::<(u32, u8)>()).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "retained zlib literal-code ownership overflow"))?)
            .and_then(|bytes| bytes.checked_add(distance_codes.capacity().checked_mul(size_of::<(u32, u8)>())?))
            .and_then(|bytes| bytes.checked_add(file_limit))
            .ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "retained zlib actual ownership overflow"))?;
        if actual > maximum_owned_bytes {
            return Err(ValueError::new(ValueRefusalKind::OwnershipLimit, "retained zlib actual ownership exceeds caller limit"));
        }
        Ok(Self { job, literal_codes, distance_codes, maximum_file_bytes, maximum_owned_bytes, adler_cursor: 0, a: 1, b: 0, raw: Vec::new(), frame: Vec::new(), frame_cursor: 0, phase: 0, complete: false, closing: false })
    }

    /// ➡️ Advances checksum, raw encoding, and framing within the caller's step budget.
    pub fn advance(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> Result<RetainedZlibStep, semio_framework_value::ValueError> {
        use semio_framework_value::{ValueError, ValueRefusalKind};
        if self.closing {
            return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "retained zlib encoder is closing"));
        }
        if self.complete {
            return Ok(RetainedZlibStep::Complete);
        }
        loop {
            if context.is_cancelled() {
                return Ok(RetainedZlibStep::Cancelled);
            }
            if context.should_yield() {
                return Ok(RetainedZlibStep::Yield);
            }
            match self.phase {
                0 => {
                    context.set_stage("deflate:zlib-checksum");
                    if self.adler_cursor < self.job.input.len() {
                        let byte = self.job.input[self.adler_cursor];
                        self.adler_cursor += 1;
                        self.a = (self.a + u32::from(byte)) % 65521;
                        self.b = (self.b + self.a) % 65521;
                        context.consume_fuel(1);
                    } else {
                        self.phase = 1;
                    }
                }
                1 => {
                    context.set_stage("deflate:zlib-encode");
                    match self.job.advance_retained(&self.literal_codes, &self.distance_codes, context) {
                        RetainedZlibStep::Yield => return Ok(RetainedZlibStep::Yield),
                        RetainedZlibStep::Cancelled => return Ok(RetainedZlibStep::Cancelled),
                        RetainedZlibStep::Complete => {
                            self.raw = self.job.take_retained_output()?;
                            let size = self.raw.len().checked_add(6).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "retained zlib result extent overflow"))?;
                            if size > self.maximum_file_bytes {
                                return Err(ValueError::new(ValueRefusalKind::OwnershipLimit, "retained zlib result exceeds caller file limit"));
                            }
                            self.frame.try_reserve_exact(size).map_err(|_| ValueError::new(ValueRefusalKind::AllocationFailed, "retained zlib frame allocation failed"))?;
                            let owned = self.owned_bytes()?;
                            if owned > self.maximum_owned_bytes {
                                return Err(ValueError::new(ValueRefusalKind::OwnershipLimit, "retained zlib actual ownership exceeds caller limit"));
                            }
                            self.frame.extend_from_slice(&[0x78, 0x01]);
                            self.phase = 2;
                        }
                    }
                }
                2 => {
                    context.set_stage("deflate:zlib-frame");
                    if self.frame_cursor < self.raw.len() {
                        self.frame.push(self.raw[self.frame_cursor]);
                        self.frame_cursor += 1;
                        context.consume_fuel(1);
                    } else {
                        self.frame.extend_from_slice(&((self.b << 16) | self.a).to_be_bytes());
                        self.complete = true;
                        self.phase = 3;
                        return Ok(RetainedZlibStep::Complete);
                    }
                }
                _ => return Ok(RetainedZlibStep::Complete),
            }
        }
    }

    pub fn progress(&self) -> (usize, usize) {
        match self.phase {
            0 => (self.adler_cursor, self.job.input.len()),
            1 => self.job.progress(),
            _ => (self.frame_cursor, self.raw.len()),
        }
    }

    pub fn owned_bytes(&self) -> Result<usize, semio_framework_value::ValueError> {
        self.job
            .retained_owned_bytes()?
            .checked_add(self.literal_codes.capacity().checked_mul(size_of::<(u32, u8)>()).ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::OwnershipLimit, "retained zlib literal-code ownership overflow"))?)
            .and_then(|bytes| bytes.checked_add(self.distance_codes.capacity().checked_mul(size_of::<(u32, u8)>())?))
            .and_then(|bytes| bytes.checked_add(self.raw.capacity()))
            .and_then(|bytes| bytes.checked_add(self.frame.capacity()))
            .ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::OwnershipLimit, "retained zlib encoder ownership overflow"))
    }

    pub fn take_output(&mut self) -> Result<Vec<u8>, semio_framework_value::ValueError> {
        if !self.complete {
            return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "retained zlib frame requested before completion"));
        }
        Ok(std::mem::take(&mut self.frame))
    }

    pub fn begin_close(&mut self) {
        self.closing = true;
        semio_framework_job::InteractiveJob::begin_close(&mut self.job);
    }

    /// ♻️ The physical bytes the next close turn releases: the encoder's own backings, then the raw job's frontier.
    pub fn next_close_release_bytes(&self) -> usize {
        match next_backing_bytes(&[&self.raw, &self.frame, &self.literal_codes, &self.distance_codes]) {
            0 => self.job.close_release_bytes(),
            bytes => bytes,
        }
    }

    /// ♻️ The flat frontier depth the next close turn needs: `1` until the encoder is terminal-empty.
    pub fn next_close_depth_demand(&self) -> usize {
        usize::from(!self.terminal_is_empty())
    }

    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        if !self.closing {
            return semio_framework_job::InteractiveJobCloseStep::Blocked;
        }
        if next_backing_bytes(&[&self.raw, &self.frame, &self.literal_codes, &self.distance_codes]) == 0 {
            return semio_framework_job::InteractiveJob::close_step(&mut self.job, grant);
        }
        if !deflate_close_admits(grant, self.next_close_release_bytes(), false) {
            return deflate_close_idle();
        }
        match release_next_backing(&mut [&mut self.raw, &mut self.frame, &mut self.literal_codes, &mut self.distance_codes]) {
            Some(released_bytes) => deflate_close_receipt(1, released_bytes),
            None => deflate_close_idle(),
        }
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.closing
            && self.raw.capacity() == 0
            && self.frame.capacity() == 0
            && self.literal_codes.capacity() == 0
            && self.distance_codes.capacity() == 0
            && semio_framework_job::InteractiveJob::terminal_is_empty(&self.job)
    }
}
//#endregion 🧵️StreamingEncode
//#endregion DeflateCodec
//#endregion 🦑️DissolvedEngineCodec
