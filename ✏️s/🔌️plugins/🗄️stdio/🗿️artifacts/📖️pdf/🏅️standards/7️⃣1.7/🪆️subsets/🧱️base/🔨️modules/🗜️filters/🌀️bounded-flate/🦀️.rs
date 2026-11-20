//! 🌀️ Resumes PDF Flate streams without claiming a flat decoded buffer.

use semio_framework_deflate::{InflateOutcome, Inflater, RetainedInflateCloseStep};
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};

#[path = "🧮️predictor/🦀️.rs"]
mod predictor;
pub use predictor::PdfPredictorCursor;

/// 🎟️ Work and retained history allocation admitted for one Flate turn.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FlateGrant {
    pub work: usize,
    pub allocation_bytes: usize,
}

/// 🚦️ The next action after a bounded Flate turn.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlateStatus {
    Yield,
    NeedsHistory { bytes: usize },
    NeedsOutput,
    Complete,
    Cancelled,
}

/// 🎫️ Work and allocation charged even when a Flate turn refuses its input.
#[derive(Debug)]
#[must_use]
pub struct FlateStep {
    pub status: Result<FlateStatus, ValueError>,
    pub work: usize,
    pub allocated_bytes: usize,
    pub written_bytes: usize,
}

/// 🧾️ The bounded effects of retiring a Flate cursor.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FlateCloseStep {
    pub work: usize,
    pub released_items: usize,
    pub released_bytes: usize,
    pub complete: bool,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum FlateFraming {
    Zlib,
    Raw,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Lifecycle {
    Decoding,
    Complete,
    Failed,
    Cancelled,
    Closing,
    Closed,
}

/// 🪈️ Decodes one admitted RFC 1950 Flate stream in bounded turns. Finish bounded retirement before dropping an admitted decoder.
#[must_use]
pub struct PdfFlateCursor {
    inflater: Option<Inflater>,
    framing: FlateFraming,
    source_len: Option<usize>,
    source_offset: usize,
    pending_input: Option<u8>,
    expanded_bytes: usize,
    maximum_output_bytes: usize,
    adler_a: u32,
    adler_b: u32,
    lifecycle: Lifecycle,
}

impl PdfFlateCursor {
    /// 🌱️ Creates a cursor without allocating compressed or expanded storage.
    pub fn new(maximum_output_bytes: usize) -> Self {
        Self::with_framing(maximum_output_bytes, FlateFraming::Zlib)
    }

    fn with_framing(maximum_output_bytes: usize, framing: FlateFraming) -> Self {
        Self {
            inflater: None,
            framing,
            source_len: None,
            source_offset: 0,
            pending_input: None,
            expanded_bytes: 0,
            maximum_output_bytes,
            adler_a: 1,
            adler_b: 0,
            lifecycle: Lifecycle::Decoding,
        }
    }

    /// ➡️ Advances against the same admitted source on each turn and fills caller-owned staging space.
    /// The caller may publish the output after a checksum-verified completion.
    pub fn advance(&mut self, source: &[u8], output: &mut [u8], grant: FlateGrant, cancelled: bool) -> FlateStep {
        let mut step = FlateStep { status: Ok(FlateStatus::Yield), work: 0, allocated_bytes: 0, written_bytes: 0 };
        step.status = match self.lifecycle {
            Lifecycle::Decoding if cancelled => {
                self.lifecycle = Lifecycle::Cancelled;
                Ok(FlateStatus::Cancelled)
            }
            Lifecycle::Decoding => self.decode(source, output, grant, &mut step),
            Lifecycle::Complete => Ok(FlateStatus::Complete),
            Lifecycle::Cancelled => Ok(FlateStatus::Cancelled),
            Lifecycle::Failed | Lifecycle::Closing | Lifecycle::Closed => Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "PDF Flate cursor is unavailable")),
        };
        if self.lifecycle == Lifecycle::Decoding && step.status.is_err() {
            self.lifecycle = Lifecycle::Failed;
        }
        step
    }

    fn decode(&mut self, source: &[u8], output: &mut [u8], grant: FlateGrant, step: &mut FlateStep) -> Result<FlateStatus, ValueError> {
        if self.source_len.is_some_and(|length| length != source.len()) {
            return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "PDF Flate source extent changed"));
        }
        while step.work < grant.work {
            let Some(inflater) = &mut self.inflater else {
                step.work += 1;
                let (offset, window) = match self.framing {
                    FlateFraming::Zlib => (2, zlib_window(source)?),
                    FlateFraming::Raw => (0, semio_framework_deflate::RETAINED_INFLATE_WINDOW_BYTES),
                };
                self.inflater = Some(Inflater::try_new_retained(window, window).map_err(inflate_error)?);
                self.source_len = Some(source.len());
                self.source_offset = offset;
                continue;
            };
            if let Some(bytes) = inflater.next_retained_allocation_bytes() {
                let available = grant.allocation_bytes - step.allocated_bytes;
                if available < bytes {
                    return Ok(FlateStatus::NeedsHistory { bytes });
                }
                step.work += 1;
                match inflater.reserve_retained_history(available) {
                    Ok(allocation) => step.allocated_bytes += allocation.allocated_bytes,
                    Err(error) => {
                        step.allocated_bytes += error.allocated_bytes;
                        return Err(ValueError::literal(error.kind, error.reason));
                    }
                }
                continue;
            }
            if step.written_bytes == output.len() && self.expanded_bytes < self.maximum_output_bytes {
                return Ok(FlateStatus::NeedsOutput);
            }
            let raw_end = source.len() - if self.framing == FlateFraming::Zlib { 4 } else { 0 };
            if self.pending_input.is_none() && self.source_offset < raw_end {
                self.pending_input = Some(source[self.source_offset]);
                self.source_offset += 1;
            }
            step.work += 1;
            match inflater.advance(&mut self.pending_input, self.source_offset == raw_end).map_err(inflate_error)? {
                InflateOutcome::NeedInput => {}
                InflateOutcome::Wrote(byte) => {
                    if self.expanded_bytes == self.maximum_output_bytes {
                        return Err(ValueError::literal(ValueRefusalKind::WorkLimit, "PDF Flate output exceeds the admitted limit"));
                    }
                    output[step.written_bytes] = byte;
                    step.written_bytes += 1;
                    self.expanded_bytes += 1;
                    self.adler_a = (self.adler_a + u32::from(byte)) % 65521;
                    self.adler_b = (self.adler_b + self.adler_a) % 65521;
                }
                InflateOutcome::Done => {
                    if self.framing == FlateFraming::Zlib {
                        let unused = inflater.terminal_unused_bytes().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "PDF Flate terminal state is absent"))?;
                        if self.pending_input.is_some() || self.source_offset.checked_sub(unused) != Some(raw_end) {
                            return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "PDF Flate payload has trailing data"));
                        }
                        let expected = u32::from_be_bytes(source[raw_end..].try_into().expect("four-byte RFC 1950 checksum"));
                        self.source_offset = source.len();
                        if (self.adler_b << 16) | self.adler_a != expected {
                            return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "PDF Flate Adler-32 checksum differs"));
                        }
                    }
                    self.lifecycle = Lifecycle::Complete;
                    return Ok(FlateStatus::Complete);
                }
            }
        }
        Ok(FlateStatus::Yield)
    }

    /// 📈️ Compressed bytes read and expanded bytes staged across all completed turns.
    pub fn progress(&self) -> (usize, usize) {
        (self.source_offset, self.expanded_bytes)
    }

    /// 🗃️ Allocated decoder history held by the cursor.
    pub fn owned_bytes(&self) -> usize {
        self.inflater.as_ref().map_or(0, Inflater::retained_allocated_bytes)
    }

    /// 🧹️ Stops decoding and begins bounded retirement.
    pub fn begin_close(&mut self) {
        if self.lifecycle != Lifecycle::Closed {
            self.lifecycle = Lifecycle::Closing;
        }
    }

    /// 📏️ Quotes the next retirement frontier, including any physical release.
    pub fn retirement_demand(&self) -> RetirementDemand {
        self.inflater.as_ref().map_or_else(
            || RetirementDemand { depth: usize::from(!self.terminal_is_empty()), ..RetirementDemand::default() },
            Inflater::retained_retirement_demands,
        )
    }

    /// ♻️ Retires at most one decoder phase within the item and release grants.
    pub fn close_step(&mut self, maximum_items: usize, maximum_release_bytes: usize) -> FlateCloseStep {
        if self.lifecycle != Lifecycle::Closing {
            return FlateCloseStep { complete: self.terminal_is_empty(), ..FlateCloseStep::default() };
        }
        let Some(inflater) = &mut self.inflater else {
            if maximum_items == 0 {
                return FlateCloseStep::default();
            }
            self.lifecycle = Lifecycle::Closed;
            return FlateCloseStep { work: 1, released_items: 1, complete: true, ..FlateCloseStep::default() };
        };
        let close = inflater.close_retained_step(maximum_items, maximum_release_bytes);
        let mut receipt = FlateCloseStep { work: inflater.last_retained_close_work() as usize, ..FlateCloseStep::default() };
        if let RetainedInflateCloseStep::Pending { released_items, released_bytes } = close {
            receipt.released_items = released_items;
            receipt.released_bytes = released_bytes;
        }
        if inflater.retained_terminal_is_empty() {
            self.inflater = None;
            self.lifecycle = Lifecycle::Closed;
            receipt.complete = true;
        }
        receipt
    }

    /// 🧺️ Confirms that the cursor retains no decoder resources after retirement.
    pub fn terminal_is_empty(&self) -> bool {
        self.lifecycle == Lifecycle::Closed
    }
}

/// 🪢️ Decodes PDF Flate, accepting recoverable raw DEFLATE framing.
pub fn decode_compatible(source: &[u8]) -> Result<Vec<u8>, ValueError> {
    decode_with_framing_recovery(source, drain_flate)
}

/// 🧮️ Decodes and reverses PDF prediction with the same Flate framing recovery.
pub fn decode_predicted_compatible(source: &[u8], predictor: u32, columns: u32, colors: u32, bits: u32) -> Result<Vec<u8>, ValueError> {
    let parameters = predictor::PredictorParameters { predictor, columns, colors, bits };
    decode_with_framing_recovery(source, |source, framing| {
        let mut decoder = PdfPredictorCursor::with_framing(usize::MAX, usize::MAX, parameters, framing)?;
        let result = collect_output(|scratch, grant| decoder.advance(source, scratch, grant, false));
        decoder.begin_close();
        while !decoder.terminal_is_empty() {
            decoder.close_step(1, decoder.retirement_demand().release_bytes);
        }
        result
    })
}

fn decode_with_framing_recovery(source: &[u8], mut decode: impl FnMut(&[u8], FlateFraming) -> Result<Vec<u8>, ValueError>) -> Result<Vec<u8>, ValueError> {
    if source.is_empty() {
        return Ok(Vec::new());
    }
    let format_refusal = |error: &ValueError| matches!(error.kind, ValueRefusalKind::InvalidValue | ValueRefusalKind::UnsupportedOwner);
    let strict = match decode(source, FlateFraming::Zlib) {
        Ok(decoded) => return Ok(decoded),
        Err(error) if format_refusal(&error) => error,
        Err(error) => return Err(error),
    };
    let has_header = source.len() > 2 && source[0] & 0x0f == 8 && u16::from_be_bytes([source[0], source[1]]).is_multiple_of(31);
    let body = if has_header { &source[2..] } else { source };
    match decode(body, FlateFraming::Raw) {
        Err(error) if format_refusal(&error) => decode(&body[..body.len().saturating_sub(4)], FlateFraming::Raw)
            .map_err(|error| if format_refusal(&error) { strict } else { error }),
        decoded => decoded,
    }
}

fn drain_flate(source: &[u8], framing: FlateFraming) -> Result<Vec<u8>, ValueError> {
    let mut decoder = PdfFlateCursor::with_framing(usize::MAX, framing);
    let result = collect_output(|scratch, grant| decoder.advance(source, scratch, grant, false));
    decoder.begin_close();
    while !decoder.terminal_is_empty() {
        decoder.close_step(1, decoder.retirement_demand().release_bytes);
    }
    result
}

fn collect_output(mut advance: impl FnMut(&mut [u8], FlateGrant) -> FlateStep) -> Result<Vec<u8>, ValueError> {
    const OUTPUT_BLOCK_BYTES: usize = 4096;
    let mut scratch = [0; OUTPUT_BLOCK_BYTES];
    let mut decoded = Vec::new();
    let grant = FlateGrant { work: OUTPUT_BLOCK_BYTES * 2, allocation_bytes: usize::MAX };
    loop {
        let turn = advance(&mut scratch, grant);
        let status = turn.status?;
        decoded.try_reserve(turn.written_bytes).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "PDF Flate output allocation failed"))?;
        decoded.extend_from_slice(&scratch[..turn.written_bytes]);
        match status {
            FlateStatus::Complete => return Ok(decoded),
            FlateStatus::Yield | FlateStatus::NeedsOutput => {}
            FlateStatus::NeedsHistory { .. } | FlateStatus::Cancelled => {
                return Err(ValueError::invariant("PDF Flate synchronous turn was refused"));
            }
        }
    }
}

/// 🧊️ Encodes TIFF horizontal differences at the requested sample width.
pub fn encode_tiff(data: &[u8], columns: u32, colors: u32, bits: u32) -> Result<Vec<u8>, ValueError> {
    predictor::transform_tiff(data, columns, colors, bits, true)
}

fn zlib_window(source: &[u8]) -> Result<usize, ValueError> {
    if source.len() < 6 {
        return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "PDF Flate frame is too short"));
    }
    let cmf = source[0];
    let flg = source[1];
    if cmf & 15 != 8 || cmf >> 4 > 7 || !u16::from_be_bytes([cmf, flg]).is_multiple_of(31) {
        return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "invalid PDF Flate zlib header"));
    }
    if flg & 32 != 0 {
        return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "PDF Flate preset dictionaries are unsupported"));
    }
    Ok(1usize << ((cmf >> 4) + 8))
}

fn inflate_error(error: semio_framework_deflate::DeflateError) -> ValueError {
    use semio_framework_deflate::DeflateError;
    let (kind, reason) = match error {
        DeflateError::BadBlockType => (ValueRefusalKind::InvalidValue, "invalid PDF Flate block kind"),
        DeflateError::BadStoredLength => (ValueRefusalKind::InvalidValue, "invalid PDF Flate stored block length"),
        DeflateError::BadHuffmanCode => (ValueRefusalKind::InvalidValue, "invalid PDF Flate Huffman code"),
        DeflateError::BadDistance => (ValueRefusalKind::InvalidValue, "invalid PDF Flate back-reference"),
        DeflateError::UnexpectedEnd => (ValueRefusalKind::InvalidValue, "truncated PDF Flate payload"),
        DeflateError::OutputLimitExceeded => (ValueRefusalKind::OwnershipLimit, "PDF Flate inflate history limit exceeded"),
    };
    ValueError::literal(kind, reason)
}

/// 📐️ Selects the nearest Paeth predictor for an eight-bit byte.
pub fn paeth(a: u8, b: u8, c: u8) -> u8 {
    let (a, b, c) = (a as i32, b as i32, c as i32);
    let p = a + b - c;
    let pa = (p - a).abs();
    let pb = (p - b).abs();
    let pc = (p - c).abs();
    if pa <= pb && pa <= pc {
        a as u8
    } else if pb <= pc {
        b as u8
    } else {
        c as u8
    }
}
