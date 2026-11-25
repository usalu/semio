//! 📥️ Custody of decoded PDF stream bytes during a retained import.

use crate::{FlateGrant, FlateStatus, PdfFlateCursor, PdfLzwCursor, PdfPredictorCursor};
use semio_framework_value::{paged::PagedBytes, ByteReadSource, ValueError, ValueRefusalKind};

/// 🧩️ Decoded stream octets in separately admitted pages; identical to the snapshot's stream payload type.
pub type PdfStreamBytes = PagedBytes<{ isize::MAX as usize }>;

#[path = "✂️packet-filters/🦀️.rs"]
mod packet_filters;
use packet_filters::SimpleFilterCursor;
#[path = "🔡️ascii85/🦀️.rs"]
mod ascii85;
use ascii85::Ascii85Cursor;

/// 🎟️ Admits source work, new capacity and backing release independently.
#[derive(Clone, Copy, Debug, Default)]
pub struct StreamDataGrant {
    pub work: usize,
    pub allocation_bytes: usize,
    pub release_bytes: usize,
}

/// 🚦️ Reports the next admissible stream data action.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StreamDataStatus {
    More,
    NeedsAllocation { bytes: usize },
    NeedsRelease { bytes: usize },
    Complete,
    Cancelled,
    Closed,
}

/// 🧾️ Accounts for work and each retained-capacity effect, including refused turns.
#[derive(Debug)]
#[must_use]
pub struct StreamDataReceipt {
    pub status: Result<StreamDataStatus, ValueError>,
    pub work: usize,
    pub allocated_bytes: usize,
    pub released_bytes: usize,
}

enum Decoder {
    Identity,
    Simple(SimpleFilterCursor),
    Ascii85(Ascii85Cursor),
    Flate(PdfFlateCursor),
    Lzw(PdfLzwCursor),
    Predicted(PdfPredictorCursor),
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Phase { Decode, Complete, Failed, Cancelled, Closing, Closed }

/// 🪣️ Owns staged output until validation succeeds and an enclosing parser takes it.
#[must_use]
pub struct PdfStreamDataCursor {
    decoder: Decoder,
    phase: Phase,
    output: PdfStreamBytes,
    staging: [u8; 256],
    staging_start: usize,
    staging_end: usize,
    input: usize,
    published: usize,
    source_len: Option<usize>,
    maximum_output: usize,
    decoder_done: bool,
}

impl PdfStreamDataCursor {
    /// 📄️ Retains an uncompressed body under the same output grants as decoded streams.
    pub fn identity(maximum_output: usize) -> Self { Self::new(Decoder::Identity, maximum_output) }

    pub fn ascii_hex(maximum_output: usize) -> Self { Self::new(Decoder::Simple(SimpleFilterCursor::ascii_hex()), maximum_output) }
    pub fn ascii85(maximum_output: usize) -> Self { Self::new(Decoder::Ascii85(Ascii85Cursor::new()), maximum_output) }
    pub fn run_length(maximum_output: usize) -> Self { Self::new(Decoder::Simple(SimpleFilterCursor::run_length()), maximum_output) }

    /// 🌀️ Retains a checksum-verified Flate body.
    pub fn flate(maximum_output: usize) -> Self { Self::new(Decoder::Flate(PdfFlateCursor::new(maximum_output)), maximum_output) }

    /// 🧮️ Retains a Flate body after reversing its selected predictor.
    pub fn flate_predicted(maximum_inflated: usize, maximum_output: usize, predictor: u32, columns: u32, colors: u32, bits: u32) -> Result<Self, ValueError> {
        let decoder = PdfPredictorCursor::flate(maximum_inflated, maximum_output, predictor, columns, colors, bits)?;
        Ok(Self::new(Decoder::Predicted(decoder), maximum_output))
    }

    /// 🧬 Retains a bounded LZW body with its selected code width law.
    pub fn lzw(maximum_output: usize, early_change: bool) -> Self {
        Self::new(Decoder::Lzw(PdfLzwCursor::new(maximum_output, early_change)), maximum_output)
    }

    /// 🧮 Retains LZW output after reversing a PNG or TIFF predictor.
    pub fn lzw_predicted(maximum_inflated: usize, maximum_output: usize, early_change: bool, predictor: u32, columns: u32, colors: u32, bits: u32) -> Result<Self, ValueError> {
        let decoder = PdfPredictorCursor::lzw(maximum_inflated, maximum_output, early_change, predictor, columns, colors, bits)?;
        Ok(Self::new(Decoder::Predicted(decoder), maximum_output))
    }

    fn new(decoder: Decoder, maximum_output: usize) -> Self {
        Self {
            decoder, phase: Phase::Decode, output: PdfStreamBytes::default(),
            staging: [0; 256], staging_start: 0, staging_end: 0, input: 0,
            published: 0, source_len: None, maximum_output, decoder_done: false,
        }
    }

    /// 📊️ Returns consumed input and retained final output. Staged scratch bytes are excluded.
    pub fn progress(&self) -> (usize, usize) { (self.input, self.published) }

    /// ⤵️ Moves validated paged output once. Finish cursor retirement separately.
    pub fn take_output(&mut self) -> Option<PdfStreamBytes> {
        if self.phase != Phase::Complete { return None; }
        self.phase = Phase::Closing;
        self.begin_decoder_close();
        Some(std::mem::take(&mut self.output))
    }

    /// 📄️ Whether the next units only decode body bytes into staging or the decoder history and move staged bytes into reserved output pages: at most two bytes of copy per unit, every reservation reported as its own allocation.
    pub fn decodes_body_bytes(&self) -> bool { self.phase == Phase::Decode }

    /// 🪣️ Quotes the fresh history unit before copying any staged stream bytes.
    pub fn next_reserves_history(&self) -> bool {
        if self.phase != Phase::Decode || self.staging_start < self.staging_end || self.decoder_done { return false; }
        match &self.decoder {
            Decoder::Flate(cursor) => cursor.next_reserves_history(),
            Decoder::Predicted(cursor) => cursor.next_reserves_history(),
            _ => false,
        }
    }

    /// ▶️ Drives one bounded turn against the same stable, already-framed source.
    pub fn advance(&mut self, source: &(impl ByteReadSource + ?Sized), grant: StreamDataGrant, cancelled: bool) -> StreamDataReceipt {
        let mut receipt = StreamDataReceipt { status: Ok(StreamDataStatus::More), work: 0, allocated_bytes: 0, released_bytes: 0 };
        if self.phase == Phase::Decode && cancelled { self.phase = Phase::Cancelled; }
        receipt.status = match self.phase {
            Phase::Decode => self.drive(source, grant, &mut receipt),
            Phase::Complete => Ok(StreamDataStatus::Complete),
            Phase::Cancelled => Ok(StreamDataStatus::Cancelled),
            Phase::Failed => Err(ValueError::invariant("PDF stream data drive already failed")),
            Phase::Closing => Ok(StreamDataStatus::More),
            Phase::Closed => Ok(StreamDataStatus::Closed),
        };
        if receipt.status.is_err() { self.phase = Phase::Failed; }
        receipt
    }

    fn drive(&mut self, source: &(impl ByteReadSource + ?Sized), grant: StreamDataGrant, receipt: &mut StreamDataReceipt) -> Result<StreamDataStatus, ValueError> {
        while receipt.work < grant.work {
            if *self.source_len.get_or_insert(source.byte_len()) != source.byte_len() {
                return Err(ValueError::invariant("PDF stream source length changed"));
            }
            if self.staging_start < self.staging_end {
                if self.output.cold_free_slots() == 0 {
                    if self.output.len() >= self.maximum_output {
                        return Err(ValueError::literal(ValueRefusalKind::WorkLimit, "PDF stream data output ceiling exceeded"));
                    }
                    let page = self.output.cold_reservation_bytes()?;
                    if page > grant.allocation_bytes - receipt.allocated_bytes {
                        return Ok(StreamDataStatus::NeedsAllocation { bytes: page });
                    }
                    receipt.work += 1;
                    let (allocated, released) = self.output.cold_reserve_page(page)?;
                    receipt.allocated_bytes += allocated;
                    receipt.released_bytes += released;
                    continue;
                }
                let count = self.staging_end - self.staging_start;
                let count = count.min(grant.work - receipt.work).min(self.maximum_output - self.output.len());
                let placed = self.output.cold_push_slice(&self.staging[self.staging_start..self.staging_start + count]);
                self.staging_start += placed;
                self.published += placed;
                receipt.work += placed;
                continue;
            }
            if self.decoder_done {
                self.phase = Phase::Complete;
                return Ok(StreamDataStatus::Complete);
            }
            let work = grant.work - receipt.work;
            self.staging_start = 0;
            self.staging_end = 0;
            let decoder_grant = FlateGrant { work, allocation_bytes: grant.allocation_bytes - receipt.allocated_bytes };
            let outcome = match &mut self.decoder {
                Decoder::Identity => {
                    if source.byte_len() > self.maximum_output {
                        return Err(ValueError::literal(ValueRefusalKind::WorkLimit, "PDF stream data output ceiling exceeded"));
                    }
                    let count = work.min(source.byte_len() - self.input).min(self.staging.len());
                    if source.read_bytes(self.input, &mut self.staging[..count]) != count {
                        receipt.work += count;
                        return Err(ValueError::invariant("PDF stream source lost an admitted fragment"));
                    }
                    self.input += count;
                    receipt.work += count;
                    self.staging_end = count;
                    self.decoder_done = self.input == source.byte_len();
                    continue;
                }
                Decoder::Simple(decoder) => {
                    let step = decoder.advance(source, &mut self.staging, work);
                    self.input = step.input;
                    self.staging_end = step.written;
                    self.decoder_done = step.complete;
                    receipt.work += step.work;
                    if step.source_changed { return Err(ValueError::invariant("PDF packet stream source lost an admitted byte")); }
                    continue;
                }
                Decoder::Ascii85(decoder) => {
                    let step = decoder.advance(source, &mut self.staging, work);
                    self.input = step.input;
                    self.staging_end = step.written;
                    receipt.work += step.work;
                    if step.source_changed { return Err(ValueError::invariant("PDF packet stream source lost an admitted byte")); }
                    self.decoder_done = step.done.map_err(|reason| ValueError::literal(ValueRefusalKind::InvalidValue, reason))?;
                    continue;
                }
                Decoder::Flate(decoder) => {
                    let outcome = decoder.advance(source, &mut self.staging, decoder_grant, false);
                    self.input = decoder.progress().0;
                    outcome
                }
                Decoder::Lzw(decoder) => {
                    let outcome = decoder.advance(source, &mut self.staging, decoder_grant, false);
                    self.input = decoder.progress().0;
                    outcome
                }
                Decoder::Predicted(decoder) => {
                    let outcome = decoder.advance(source, &mut self.staging, decoder_grant, false);
                    self.input = decoder.progress().0;
                    outcome
                }
            };
            self.staging_end = outcome.written_bytes;
            receipt.work += outcome.work;
            receipt.allocated_bytes += outcome.allocated_bytes;
            match outcome.status? {
                FlateStatus::NeedsHistory { bytes } if self.staging_end == 0 => return Ok(StreamDataStatus::NeedsAllocation { bytes }),
                FlateStatus::NeedsHistory { .. } | FlateStatus::NeedsOutput | FlateStatus::Yield => {
                    if outcome.work == 0 && self.staging_end == 0 { return Ok(StreamDataStatus::More); }
                }
                FlateStatus::Complete => self.decoder_done = true,
                FlateStatus::Cancelled => return Err(ValueError::invariant("PDF stream data child cancelled during an active turn")),
            }
        }
        Ok(StreamDataStatus::More)
    }

    /// ♻️ Begins retirement after success, cancellation or refusal.
    pub fn begin_close(&mut self) {
        if matches!(self.phase, Phase::Closing | Phase::Closed) { return; }
        self.phase = Phase::Closing;
        self.begin_decoder_close();
    }

    fn begin_decoder_close(&mut self) {
        match &mut self.decoder {
            Decoder::Identity | Decoder::Simple(_) | Decoder::Ascii85(_) => {}
            Decoder::Flate(decoder) => decoder.begin_close(),
            Decoder::Lzw(decoder) => decoder.begin_close(),
            Decoder::Predicted(decoder) => decoder.begin_close(),
        }
    }

    /// 🧹️ Releases one pending backing action at a time while work and bytes fit.
    pub fn close_step(&mut self, grant: StreamDataGrant) -> StreamDataReceipt {
        let mut receipt = StreamDataReceipt { status: Ok(StreamDataStatus::More), work: 0, allocated_bytes: 0, released_bytes: 0 };
        while self.phase == Phase::Closing && receipt.work < grant.work {
            if !self.output.retained_bytes().terminal_is_empty() {
                let step = match self.output.cold_retire_page(grant.release_bytes - receipt.released_bytes) {
                    Ok(step) => step,
                    Err(error) => { receipt.status = Err(error); return receipt; }
                };
                if let Some(bytes) = step.needs_release {
                    receipt.status = Ok(StreamDataStatus::NeedsRelease { bytes });
                    return receipt;
                }
                receipt.work += step.work;
                receipt.released_bytes += step.released_bytes;
                continue;
            }
            let (demand, step) = match &mut self.decoder {
                Decoder::Identity | Decoder::Simple(_) | Decoder::Ascii85(_) => { self.phase = Phase::Closed; break; }
                Decoder::Flate(decoder) => (decoder.retirement_demand().release_bytes, decoder.close_step(1, grant.release_bytes - receipt.released_bytes)),
                Decoder::Lzw(decoder) => (decoder.retirement_demand().release_bytes, decoder.close_step(1, grant.release_bytes - receipt.released_bytes)),
                Decoder::Predicted(decoder) => (decoder.retirement_demand().release_bytes, decoder.close_step(1, grant.release_bytes - receipt.released_bytes)),
            };
            receipt.work += step.work;
            receipt.released_bytes += step.released_bytes;
            if step.complete { self.phase = Phase::Closed; }
            if step.work == 0 && !step.complete {
                receipt.status = Ok(StreamDataStatus::NeedsRelease { bytes: demand });
                return receipt;
            }
        }
        if self.phase == Phase::Closed { receipt.status = Ok(StreamDataStatus::Closed); }
        receipt
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
