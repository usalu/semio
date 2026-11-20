//! 🧮️ Applies PDF PNG and TIFF predictors in bounded turns after stream decoding.

use super::*;
use crate::PdfLzwCursor;

#[derive(Clone, Copy)]
pub(super) struct PredictorParameters {
    pub predictor: u32,
    pub columns: u32,
    pub colors: u32,
    pub bits: u32,
}

struct Png {
    row_bytes: usize,
    pixel_bytes: usize,
    filter: Option<u8>,
    previous: Vec<u8>,
    current: Vec<u8>,
}

struct Tiff {
    row_samples: usize,
    colors: usize,
    bits: u32,
    encode: bool,
    sample: usize,
    high: Option<u8>,
    history: Vec<u16>,
}

enum Predictor {
    Png(Png),
    Tiff(Tiff),
}

#[derive(Default)]
enum Predicted {
    #[default]
    Empty,
    One(u8),
    Two([u8; 2]),
}

impl Predicted {
    fn next(&mut self) -> Option<u8> {
        match std::mem::take(self) {
            Self::Empty => None,
            Self::One(byte) => Some(byte),
            Self::Two([first, second]) => {
                *self = Self::One(second);
                Some(first)
            }
        }
    }
}

enum PredictorInput {
    Flate(PdfFlateCursor),
    Lzw(PdfLzwCursor),
}

impl PredictorInput {
    fn advance(&mut self, source: &[u8], output: &mut [u8], grant: FlateGrant) -> FlateStep {
        match self {
            Self::Flate(cursor) => cursor.advance(source, output, grant, false),
            Self::Lzw(cursor) => cursor.advance(source, output, grant, false),
        }
    }

    fn progress(&self) -> (usize, usize) {
        match self {
            Self::Flate(cursor) => cursor.progress(),
            Self::Lzw(cursor) => cursor.progress(),
        }
    }

    fn owned_bytes(&self) -> usize {
        match self {
            Self::Flate(cursor) => cursor.owned_bytes(),
            Self::Lzw(cursor) => cursor.owned_bytes(),
        }
    }

    fn begin_close(&mut self) {
        match self {
            Self::Flate(cursor) => cursor.begin_close(),
            Self::Lzw(cursor) => cursor.begin_close(),
        }
    }

    fn retirement_demand(&self) -> RetirementDemand {
        match self {
            Self::Flate(cursor) => cursor.retirement_demand(),
            Self::Lzw(cursor) => cursor.retirement_demand(),
        }
    }

    fn close_step(&mut self, items: usize, bytes: usize) -> FlateCloseStep {
        match self {
            Self::Flate(cursor) => cursor.close_step(items, bytes),
            Self::Lzw(cursor) => cursor.close_step(items, bytes),
        }
    }
}

/// 🧬️ Resumes a PDF byte decoder and its row predictor without owning either stream buffer.
#[must_use]
pub struct PdfPredictorCursor {
    input: PredictorInput,
    predictor: Predictor,
    filtered_byte: Option<u8>,
    predicted: Predicted,
    input_complete: bool,
    predictor_closed: bool,
    output_bytes: usize,
    maximum_output_bytes: usize,
    lifecycle: Lifecycle,
}

impl PdfPredictorCursor {
    /// 🎛️ Admits independent maximum extents for inflated predictor data and logical output.
    pub fn flate(maximum_inflated_bytes: usize, maximum_output_bytes: usize, predictor: u32, columns: u32, colors: u32, bits_per_component: u32) -> Result<Self, ValueError> {
        Self::with_framing(maximum_inflated_bytes, maximum_output_bytes, PredictorParameters { predictor, columns, colors, bits: bits_per_component }, FlateFraming::Zlib)
    }

    pub(super) fn with_framing(maximum_inflated_bytes: usize, maximum_output_bytes: usize, parameters: PredictorParameters, framing: FlateFraming) -> Result<Self, ValueError> {
        Self::start(
            PredictorInput::Flate(PdfFlateCursor::with_framing(maximum_inflated_bytes, framing)),
            maximum_output_bytes, parameters,
        )
    }

    /// 🧵 Admits LZW input and reversed predictor output under independent byte ceilings.
    pub fn lzw(maximum_inflated_bytes: usize, maximum_output_bytes: usize, early_change: bool, predictor: u32, columns: u32, colors: u32, bits: u32) -> Result<Self, ValueError> {
        Self::start(
            PredictorInput::Lzw(PdfLzwCursor::new(maximum_inflated_bytes, early_change)),
            maximum_output_bytes, PredictorParameters { predictor, columns, colors, bits },
        )
    }

    fn start(input: PredictorInput, maximum_output_bytes: usize, parameters: PredictorParameters) -> Result<Self, ValueError> {
        Ok(Self {
            input,
            predictor: Predictor::new(parameters)?,
            filtered_byte: None,
            predicted: Predicted::Empty,
            input_complete: false,
            predictor_closed: false,
            output_bytes: 0,
            maximum_output_bytes,
            lifecycle: Lifecycle::Decoding,
        })
    }

    /// ⚙️ Charges both stages and writes to caller-owned staging space. Publish only after completion.
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
            Lifecycle::Failed | Lifecycle::Closing | Lifecycle::Closed => Err(ValueError::invariant("PDF predictor cursor is unavailable")),
        };
        if self.lifecycle == Lifecycle::Decoding && step.status.is_err() {
            self.lifecycle = Lifecycle::Failed;
        }
        step
    }

    fn decode(&mut self, source: &[u8], output: &mut [u8], grant: FlateGrant, step: &mut FlateStep) -> Result<FlateStatus, ValueError> {
        while step.work < grant.work {
            let allocation = grant.allocation_bytes - step.allocated_bytes;
            if !matches!(self.predicted, Predicted::Empty) {
                if self.output_bytes == self.maximum_output_bytes {
                    return Err(ValueError::literal(ValueRefusalKind::WorkLimit, "PDF predictor output exceeds the admitted extent"));
                }
                if step.written_bytes == output.len() {
                    return Ok(FlateStatus::NeedsOutput);
                }
                output[step.written_bytes] = self.predicted.next().expect("pending predicted byte");
                step.written_bytes += 1;
                self.output_bytes += 1;
                step.work += 1;
            } else if let Some(byte) = self.filtered_byte {
                let demand = self.predictor.next_allocation();
                if demand != 0 {
                    if demand > allocation {
                        return Ok(FlateStatus::NeedsHistory { bytes: demand });
                    }
                    step.work += 1;
                    let before = self.predictor.owned_bytes();
                    let reserve = self.predictor.reserve();
                    let allocated = self.predictor.owned_bytes() - before;
                    step.allocated_bytes += allocated;
                    if allocated > allocation {
                        return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "PDF predictor allocation exceeded the granted bytes"));
                    }
                    reserve?;
                    continue;
                }
                self.filtered_byte = None;
                step.work += 1;
                self.predicted = self.predictor.push(byte)?;
            } else if self.input_complete {
                step.work += 1;
                self.predictor.finish()?;
                self.lifecycle = Lifecycle::Complete;
                return Ok(FlateStatus::Complete);
            } else {
                let mut byte = [0];
                let turn = self.input.advance(source, &mut byte, FlateGrant { work: grant.work - step.work, allocation_bytes: allocation });
                step.work += turn.work;
                step.allocated_bytes += turn.allocated_bytes;
                if turn.written_bytes != 0 {
                    self.filtered_byte = Some(byte[0]);
                }
                match turn.status? {
                    FlateStatus::Complete => self.input_complete = true,
                    FlateStatus::Yield | FlateStatus::NeedsOutput => {}
                    status @ (FlateStatus::NeedsHistory { .. } | FlateStatus::Cancelled) => return Ok(status),
                }
            }
        }
        Ok(FlateStatus::Yield)
    }

    /// 📟️ Reports compressed bytes read and final predictor bytes staged.
    pub fn progress(&self) -> (usize, usize) {
        (self.input.progress().0, self.output_bytes)
    }

    /// ⛲️ Counts decoder output bytes, including row predictor tags.
    pub fn decoded_input_bytes(&self) -> usize {
        self.input.progress().1
    }

    /// 🧳️ Returns the combined decoder and predictor allocation.
    pub fn owned_bytes(&self) -> usize {
        self.input.owned_bytes().saturating_add(self.predictor.owned_bytes())
    }

    /// 🧽️ Starts retirement of predictor rows and inflater history.
    pub fn begin_close(&mut self) {
        if self.lifecycle != Lifecycle::Closed {
            self.lifecycle = Lifecycle::Closing;
            self.input.begin_close();
        }
    }

    /// 📐️ Quotes the physical release and depth of one retirement frontier.
    pub fn retirement_demand(&self) -> RetirementDemand {
        if self.predictor_closed {
            self.input.retirement_demand()
        } else {
            RetirementDemand { release_bytes: self.predictor.next_release(), depth: 1, ..RetirementDemand::default() }
        }
    }

    /// ♻️ Releases at most one predictor buffer or one inflater phase.
    pub fn close_step(&mut self, maximum_items: usize, maximum_release_bytes: usize) -> FlateCloseStep {
        if self.lifecycle != Lifecycle::Closing {
            return FlateCloseStep { complete: self.terminal_is_empty(), ..FlateCloseStep::default() };
        }
        if !self.predictor_closed {
            let bytes = self.predictor.next_release();
            if maximum_items == 0 || bytes > maximum_release_bytes {
                return FlateCloseStep::default();
            }
            self.predictor.release_one();
            self.predictor_closed = self.predictor.owned_bytes() == 0;
            return FlateCloseStep { work: 1, released_items: 1, released_bytes: bytes, complete: false };
        }
        let step = self.input.close_step(maximum_items, maximum_release_bytes);
        if step.complete {
            self.lifecycle = Lifecycle::Closed;
        }
        step
    }

    /// 🏁️ Confirms retirement of every predictor and inflate buffer.
    pub fn terminal_is_empty(&self) -> bool {
        self.lifecycle == Lifecycle::Closed
    }
}

impl Predictor {
    fn new(parameters: PredictorParameters) -> Result<Self, ValueError> {
        let PredictorParameters { predictor, columns, colors, bits } = parameters;
        let colors = colors as usize;
        if columns == 0 || colors == 0 || !matches!(bits, 1 | 2 | 4 | 8 | 16) {
            return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "invalid PDF predictor dimensions or sample width"));
        }
        let extent_error = || ValueError::literal(ValueRefusalKind::OwnershipLimit, "PDF predictor row extent overflow");
        let row_samples = (columns as usize).checked_mul(colors).ok_or_else(extent_error)?;
        let pixel_bits = colors.checked_mul(bits as usize).ok_or_else(extent_error)?;
        match predictor {
            2 => {
                colors.checked_mul(size_of::<u16>()).ok_or_else(extent_error)?;
                Ok(Self::Tiff(Tiff { row_samples, colors, bits, encode: false, sample: 0, high: None, history: Vec::new() }))
            }
            10..=u32::MAX => {
                let row_bytes = row_samples.checked_mul(bits as usize).ok_or_else(extent_error)?.div_ceil(8);
                Ok(Self::Png(Png { row_bytes, pixel_bytes: pixel_bits.div_ceil(8), filter: None, previous: Vec::new(), current: Vec::new() }))
            }
            _ => Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "unsupported PDF row predictor")),
        }
    }

    fn next_allocation(&self) -> usize {
        match self {
            Self::Png(state) if state.previous.capacity() == 0 || state.current.capacity() == 0 => state.row_bytes,
            Self::Tiff(state) if state.history.capacity() == 0 => state.colors * size_of::<u16>(),
            _ => 0,
        }
    }

    fn reserve(&mut self) -> Result<(), ValueError> {
        let reserve = match self {
            Self::Png(state) => {
                let row = if state.previous.capacity() == 0 { &mut state.previous } else { &mut state.current };
                row.try_reserve_exact(state.row_bytes)
            }
            Self::Tiff(state) => state.history.try_reserve_exact(state.colors),
        };
        reserve.map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "PDF predictor row allocation failed"))
    }

    fn push(&mut self, byte: u8) -> Result<Predicted, ValueError> {
        match self {
            Self::Png(state) => state.push(byte),
            Self::Tiff(state) => Ok(state.push(byte)),
        }
    }

    fn finish(&self) -> Result<(), ValueError> {
        if matches!(self, Self::Tiff(state) if state.high.is_some()) {
            return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "truncated sixteen-bit TIFF predictor sample"));
        }
        Ok(())
    }

    fn owned_bytes(&self) -> usize {
        match self {
            Self::Png(state) => state.previous.capacity().saturating_add(state.current.capacity()),
            Self::Tiff(state) => state.history.capacity() * size_of::<u16>(),
        }
    }

    fn next_release(&self) -> usize {
        match self {
            Self::Png(state) if state.previous.capacity() > 0 => state.previous.capacity(),
            Self::Png(state) => state.current.capacity(),
            Self::Tiff(_) => self.owned_bytes(),
        }
    }

    fn release_one(&mut self) {
        match self {
            Self::Png(state) if state.previous.capacity() > 0 => state.previous = Vec::new(),
            Self::Png(state) => state.current = Vec::new(),
            Self::Tiff(state) => state.history = Vec::new(),
        }
    }
}

impl Png {
    fn push(&mut self, byte: u8) -> Result<Predicted, ValueError> {
        let Some(filter) = self.filter else {
            self.filter = Some(byte);
            return Ok(Predicted::Empty);
        };
        let column = self.current.len();
        let left = column.checked_sub(self.pixel_bytes);
        let a = left.and_then(|index| self.current.get(index)).copied().unwrap_or(0);
        let b = self.previous.get(column).copied().unwrap_or(0);
        let c = left.and_then(|index| self.previous.get(index)).copied().unwrap_or(0);
        let prediction = match filter {
            0 => 0,
            1 => a,
            2 => b,
            3 => ((u16::from(a) + u16::from(b)) / 2) as u8,
            4 => paeth(a, b, c),
            _ => return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "invalid PDF PNG predictor row filter")),
        };
        let decoded = byte.wrapping_add(prediction);
        self.current.push(decoded);
        if self.current.len() == self.row_bytes {
            std::mem::swap(&mut self.previous, &mut self.current);
            self.current.clear();
            self.filter = None;
        }
        Ok(Predicted::One(decoded))
    }
}

impl Tiff {
    fn push(&mut self, byte: u8) -> Predicted {
        if self.bits == 16 {
            let Some(high) = self.high.take() else {
                self.high = Some(byte);
                return Predicted::Empty;
            };
            return Predicted::Two(self.transform_sample(u16::from_be_bytes([high, byte])).to_be_bytes());
        }
        let samples = (self.row_samples - self.sample).min((8 / self.bits) as usize);
        let mask = ((1u16 << self.bits) - 1) as u8;
        let mut decoded = byte;
        for index in 0..samples {
            let shift = 8 - (index as u32 + 1) * self.bits;
            let sample = u16::from((byte >> shift) & mask);
            decoded = (decoded & !(mask << shift)) | (self.transform_sample(sample) as u8) << shift;
        }
        Predicted::One(decoded)
    }

    fn transform_sample(&mut self, sample: u16) -> u16 {
        let channel = self.sample % self.colors;
        let predicted = if self.sample < self.colors { 0 } else { self.history[channel] };
        let mask = ((1u32 << self.bits) - 1) as u16;
        let (logical, output) = if self.encode {
            (sample, sample.wrapping_sub(predicted) & mask)
        } else {
            let decoded = sample.wrapping_add(predicted) & mask;
            (decoded, decoded)
        };
        if channel == self.history.len() {
            self.history.push(logical);
        } else {
            self.history[channel] = logical;
        }
        self.sample = (self.sample + 1) % self.row_samples;
        output
    }
}

pub(super) fn transform_tiff(data: &[u8], columns: u32, colors: u32, bits: u32, encode: bool) -> Result<Vec<u8>, ValueError> {
    let mut predictor = Predictor::new(PredictorParameters { predictor: 2, columns, colors, bits })?;
    if data.is_empty() {
        return Ok(Vec::new());
    }
    predictor.reserve()?;
    if let Predictor::Tiff(state) = &mut predictor {
        state.encode = encode;
    }
    let mut output = Vec::new();
    output.try_reserve_exact(data.len()).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "TIFF predictor output allocation failed"))?;
    for &byte in data {
        let mut predicted = predictor.push(byte)?;
        while let Some(value) = predicted.next() {
            output.push(value);
        }
    }
    predictor.finish()?;
    Ok(output)
}
