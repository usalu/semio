//! 🧵️ Resumable deflate encoder of the zlib codec: the encode job record and its step, match and block emission. See [`super`].

use super::*;

//#region 🦑️DissolvedEngineCodec
//#region DeflateCodec
//#region 🧵️StreamingEncode
/// 🧵️ Persistent fixed-Huffman DEFLATE encoder with bounded LZ77 transitions and portable checkpoints.
pub struct DeflateEncodeJob {
    pub(super) input: Vec<u8>,
    pub(super) writer: BitWriter,
    pub(super) head: Vec<i32>,
    pub(super) previous: Vec<i32>,
    pub(super) position: usize,
    pub(super) pending: Option<PendingMatch>,
    pub(super) checkpoint_interval: usize,
    pub(super) next_checkpoint: usize,
    pub(super) complete: bool,
    pub(super) publication: Option<Box<Publication>>,
}

impl DeflateEncodeJob {
    /// 🌱️ Creates a deterministic encoder; `checkpoint_interval` is measured in applied input bytes.
    pub fn new(input: Vec<u8>, checkpoint_interval: usize) -> Self {
        let mut writer = BitWriter::new();
        writer.write_bits(0b011, 3);
        let checkpoint_interval = checkpoint_interval.max(1);
        Self { input, writer, head: vec![-1; HASH_SIZE], previous: vec![-1; WINDOW], position: 0, pending: None, checkpoint_interval, next_checkpoint: checkpoint_interval, complete: false, publication: None }
    }

    /// 📈 Returns applied input bytes and total input bytes.
    pub fn progress(&self) -> (usize, usize) {
        (self.position, self.input.len())
    }

    /// 💾 Encodes all persistent state required for byte-identical continuation.
    pub fn checkpoint_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(self.input.len() + self.writer.out.len() + (self.head.len() + self.previous.len()) * 4 + 96);
        bytes.extend_from_slice(DEFLATE_CHECKPOINT_MAGIC);
        write_bytes(&mut bytes, &self.input);
        write_bytes(&mut bytes, &self.writer.out);
        bytes.push(self.writer.cur);
        bytes.push(self.writer.nbits);
        write_i32s(&mut bytes, &self.head);
        write_i32s(&mut bytes, &self.previous);
        write_usize(&mut bytes, self.position);
        match self.pending {
            Some(pending) => {
                bytes.push(1);
                write_usize(&mut bytes, pending.start);
                write_usize(&mut bytes, pending.length);
                write_usize(&mut bytes, pending.distance);
            }
            None => bytes.push(0),
        }
        write_usize(&mut bytes, self.checkpoint_interval);
        write_usize(&mut bytes, self.next_checkpoint);
        bytes.push(u8::from(self.complete));
        bytes
    }

    /// ♻️ Restores a job checkpoint without replaying already-applied input.
    pub fn from_checkpoint(bytes: &[u8]) -> Result<Self, String> {
        let mut reader = CheckpointReader::new(bytes);
        if reader.take(4)? != DEFLATE_CHECKPOINT_MAGIC {
            return Err("invalid DEFLATE job checkpoint magic".into());
        }
        let input = reader.read_bytes()?;
        let output = reader.read_bytes()?;
        let cur = reader.read_u8()?;
        let nbits = reader.read_u8()?;
        if nbits > 7 {
            return Err("invalid DEFLATE job bit cursor".into());
        }
        let head = reader.read_i32s()?;
        let previous = reader.read_i32s()?;
        if head.len() != HASH_SIZE || previous.len() != WINDOW {
            return Err("invalid DEFLATE job index dimensions".into());
        }
        let position = reader.read_usize()?;
        let pending = match reader.read_u8()? {
            0 => None,
            1 => Some(PendingMatch { start: reader.read_usize()?, length: reader.read_usize()?, distance: reader.read_usize()? }),
            _ => return Err("invalid DEFLATE job pending-match tag".into()),
        };
        let checkpoint_interval = reader.read_usize()?.max(1);
        let next_checkpoint = reader.read_usize()?;
        let complete = reader.read_u8()? != 0;
        if position > input.len() || !reader.is_empty() {
            return Err("invalid DEFLATE job checkpoint cursor".into());
        }
        Ok(Self { input, writer: BitWriter { out: output, cur, nbits }, head, previous, position, pending, checkpoint_interval, next_checkpoint, complete, publication: None })
    }

    pub(super) fn insert(&mut self, position: usize) {
        if position + MIN_MATCH <= self.input.len() {
            let hash = hash3(&self.input, position);
            self.previous[position & (WINDOW - 1)] = self.head[hash];
            self.head[hash] = position as i32;
        }
    }

    fn emit_literal(&mut self, position: usize, codes: &[(u32, u8)]) {
        let (code, length) = codes[self.input[position] as usize];
        self.writer.write_bits(code, length);
    }

    fn emit_match(&mut self, pending: PendingMatch, literal_codes: &[(u32, u8)], distance_codes: &[(u32, u8)]) {
        let (symbol, extra, extra_bits) = length_symbol(pending.length);
        let (code, length) = literal_codes[symbol];
        self.writer.write_bits(code, length);
        self.writer.write_bits(extra, extra_bits);
        let (symbol, extra, extra_bits) = distance_symbol(pending.distance);
        let (code, length) = distance_codes[symbol];
        self.writer.write_bits(code, length);
        self.writer.write_bits(extra, extra_bits);
    }

    pub(super) fn process_transition(&mut self, literal_codes: &[(u32, u8)], distance_codes: &[(u32, u8)]) -> usize {
        let before = self.position;
        if self.position >= self.input.len() {
            if let Some(pending) = self.pending.take() {
                self.emit_match(pending, literal_codes, distance_codes);
            }
            let (code, length) = literal_codes[256];
            self.writer.write_bits(code, length);
            self.writer.align_byte();
            self.complete = true;
            return 1;
        }
        let next = longest_match(&self.input, self.position, &self.head, &self.previous, DEFLATE_MAX_CHAIN);
        self.insert(self.position);
        match (self.pending.take(), next) {
            (None, Some((length, distance))) => {
                self.pending = Some(PendingMatch { start: self.position, length, distance });
                self.position += 1;
            }
            (Some(pending), next) if matches!(next, Some((length, _)) if length > pending.length) => {
                self.emit_literal(pending.start, literal_codes);
                let (length, distance) = next.expect("guarded match");
                self.pending = Some(PendingMatch { start: self.position, length, distance });
                self.position += 1;
            }
            (Some(pending), _) => {
                self.emit_match(pending, literal_codes, distance_codes);
                let end = (pending.start + pending.length).min(self.input.len());
                for position in (pending.start + 2)..end {
                    self.insert(position);
                }
                self.position = end;
            }
            (None, None) => {
                self.emit_literal(self.position, literal_codes);
                self.position += 1;
            }
        }
        self.position.saturating_sub(before).max(1)
    }

    pub(super) fn finish(mut self) -> Vec<u8> {
        let literal_codes = build_codes(&fixed_lit_lengths());
        let distance_codes = build_codes(&fixed_dist_lengths());
        while !self.complete {
            self.process_transition(&literal_codes, &distance_codes);
        }
        self.writer.out
    }

    /// 🧵️ Advances the raw encoder without publishing job checkpoints.
    pub(super) fn advance_retained(
        &mut self,
        literal_codes: &[(u32, u8)],
        distance_codes: &[(u32, u8)],
        context: &mut semio_framework_job::StepContext<'_>,
    ) -> RetainedZlibStep {
        loop {
            if self.complete {
                return RetainedZlibStep::Complete;
            }
            let work = self.process_transition(&literal_codes, &distance_codes);
            context.consume_fuel(work as u64);
            if context.is_cancelled() {
                return RetainedZlibStep::Cancelled;
            }
            if context.should_yield() {
                return RetainedZlibStep::Yield;
            }
        }
    }

    pub(super) fn take_retained_output(&mut self) -> Result<Vec<u8>, semio_framework_value::ValueError> {
        if !self.complete {
            return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "retained Deflate output requested before completion"));
        }
        Ok(std::mem::take(&mut self.writer.out))
    }

    pub(super) fn retained_owned_bytes(&self) -> Result<usize, semio_framework_value::ValueError> {
        self.input
            .capacity()
            .checked_add(self.writer.out.capacity())
            .and_then(|bytes| bytes.checked_add(self.head.capacity().checked_mul(size_of::<i32>())?))
            .and_then(|bytes| bytes.checked_add(self.previous.capacity().checked_mul(size_of::<i32>())?))
            .ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::OwnershipLimit, "retained Deflate ownership overflow"))
    }

    /// 📸️ Freezes the checkpoint at the current cursor and stages its publication.
    pub(super) fn stage_checkpoint(&mut self) {
        self.publication = Some(Publication::new(PublicationKind::Checkpoint(self.position as u64), self.checkpoint_bytes(), Vec::new()));
    }

    /// 🏁️ Freezes the final state and output and stages their publication.
    pub(super) fn stage_commit(&mut self) {
        self.publication = Some(Publication::new(PublicationKind::Commit, self.checkpoint_bytes(), self.writer.out.clone()));
    }
}
//#endregion 🧵️StreamingEncode
//#endregion DeflateCodec
//#endregion 🦑️DissolvedEngineCodec
