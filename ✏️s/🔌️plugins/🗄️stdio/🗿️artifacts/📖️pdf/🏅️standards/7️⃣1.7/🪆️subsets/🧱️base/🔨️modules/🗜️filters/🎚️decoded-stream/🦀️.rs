//! 🧾️ Admits a parsed PDF stream dictionary and its borrowed body to the controlled decoder.

use super::*;
use semio_framework_value::{ValueError, ValueRefusalKind};
use std::mem::size_of;

/// 📬️ Validated bytes and typed declarations for the held stream.
pub struct DecodedStreamData {
    pub bytes: Vec<u8>,
    pub filters: Vec<PdfStreamFilter>,
}

/// 🫙️ Scans the stream declaration and retains decoded bytes until every filter validates.
#[must_use]
pub struct PdfDecodedStreamCursor {
    fields: [Option<usize>; 5],
    position: usize,
    parameters: [Option<u32>; 5],
    parameter_position: usize,
    stage: usize,
    stages: Option<usize>,
    checked_stages: usize,
    decoder: Option<PdfStreamDataCursor>,
    filters: Vec<PdfStreamFilter>,
    input: Option<Vec<u8>>,
    next_input: Option<Vec<u8>>,
    consumed: usize,
    maximum_inflated: usize,
    maximum_output: usize,
    finishing_stage: bool,
    complete: bool,
    cancelled: bool,
    closing: bool,
}

impl PdfDecodedStreamCursor {
    /// 🌱️ Creates a stream plan before visiting the caller-owned dictionary or source.
    pub fn new(maximum_inflated: usize, maximum_output: usize) -> Self {
        Self {
            fields: [None; 5], position: 0, parameters: [None; 5], parameter_position: 0,
            stage: 0, stages: None, checked_stages: 0, decoder: None, filters: Vec::new(), input: None,
            next_input: None, consumed: 0, maximum_inflated, maximum_output,
            finishing_stage: false, complete: false, cancelled: false, closing: false,
        }
    }

    /// 🧭️ Reports original source progress and assembled final output.
    pub fn progress(&self) -> (usize, usize) {
        let output = if self.is_last_stage() { self.decoder.as_ref().map_or(0, |decoder| decoder.progress().1) } else { 0 };
        (self.consumed, output)
    }

    fn is_last_stage(&self) -> bool {
        self.stages.is_some_and(|stages| self.stage + 1 >= stages)
    }

    /// 🎚️ Quotes only the fresh history unit of the currently decoding filter.
    pub fn next_reserves_history(&self) -> bool {
        !self.cancelled && !self.closing && !self.complete && !self.finishing_stage
            && self.decoder.as_ref().is_some_and(PdfStreamDataCursor::next_reserves_history)
    }

    /// ▶️ Drives dictionary planning and the ordered filter stages over the stable borrowed body.
    pub fn advance(&mut self, dict: &[PdfDictEntry], raw: &[u8], mut grant: StreamDataGrant, cancelled: bool) -> StreamDataReceipt {
        self.cancelled |= cancelled;
        if self.cancelled && !self.closing { return empty(Ok(StreamDataStatus::Cancelled), 0); }
        if self.closing { return empty(Ok(StreamDataStatus::More), 0); }
        if self.complete { return empty(Ok(StreamDataStatus::Complete), 0); }
        if self.finishing_stage { return self.advance_handoff(grant); }

        let mut receipt = empty(Ok(StreamDataStatus::More), 0);
        while self.decoder.is_none() && grant.work > 0 {
            receipt.work += 1;
            grant.work -= 1;
            match self.plan_one(dict, grant.allocation_bytes, &mut receipt) {
                Ok(StreamDataStatus::More) => {}
                status => { receipt.status = status; return receipt; }
            }
        }
        let Some(decoder) = self.decoder.as_mut() else { return receipt };
        grant.allocation_bytes -= receipt.allocated_bytes;
        let source = self.input.as_deref().unwrap_or(raw);
        let mut decoded = decoder.advance(source, grant, false);
        decoded.work += receipt.work;
        decoded.allocated_bytes += receipt.allocated_bytes;
        if self.stage == 0 { self.consumed = decoder.progress().0; }
        if matches!(decoded.status, Ok(StreamDataStatus::Complete)) {
            if self.is_last_stage() {
                self.complete = true;
            } else {
                self.finishing_stage = true;
                decoded.status = Ok(StreamDataStatus::More);
            }
        }
        decoded
    }

    fn plan_one(&mut self, dict: &[PdfDictEntry], allocation: usize, receipt: &mut StreamDataReceipt) -> Result<StreamDataStatus, ValueError> {
        if let Some(entry) = dict.get(self.position) {
            if let Some(slot) = ["Filter", "F", "DecodeParms", "DP", "Subtype"].iter().position(|&key| key == entry.key) {
                self.fields[slot].get_or_insert(self.position);
            }
            self.position += 1;
            return Ok(StreamDataStatus::More);
        }
        let value = |first: usize, alias: usize| {
            self.fields[first].or(self.fields[alias]).and_then(|index| dict.get(index)).map(|entry| &entry.value)
        };
        let declaration = value(0, 1);
        let Some(stages) = self.stages else {
            self.stages = Some(match declaration {
                None | Some(PdfObject::Null) => 0,
                Some(PdfObject::Name(_)) => 1,
                Some(PdfObject::Array(filters)) => filters.len(),
                _ => return Err(unsupported()),
            });
            return Ok(StreamDataStatus::More);
        };
        if self.checked_stages < stages {
            if matches!(planned_filter(declaration, self.checked_stages)?, PlannedFilter::Dct)
                && (stages != 1 || value(4, 4).and_then(PdfObject::as_name) != Some("Image")) {
                return Err(unsupported());
            }
            self.checked_stages += 1;
            return Ok(StreamDataStatus::More);
        }
        if stages != 0 && self.filters.capacity() == 0 {
            let bytes = stages.checked_mul(size_of::<PdfStreamFilter>()).ok_or_else(|| invariant("PDF stream filter table size overflow"))?;
            if bytes > allocation - receipt.allocated_bytes { return Ok(StreamDataStatus::NeedsAllocation { bytes }); }
            self.filters.try_reserve_exact(stages).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "PDF stream filter table allocation failed"))?;
            receipt.allocated_bytes += self.filters.capacity() * size_of::<PdfStreamFilter>();
            return Ok(StreamDataStatus::More);
        }
        let filter = planned_filter(declaration, self.stage)?;
        let maximum_output = if self.is_last_stage() { self.maximum_output } else { self.maximum_inflated };
        let parameter = match value(2, 3) {
            Some(PdfObject::Array(parameters)) => parameters.get(self.stage),
            parameter if self.stage == 0 => parameter,
            _ => None,
        };
        let (decoder, filter) = match filter {
            PlannedFilter::Identity => (PdfStreamDataCursor::identity(maximum_output), None),
            PlannedFilter::AsciiHex => (PdfStreamDataCursor::ascii_hex(maximum_output), Some(PdfStreamFilter::AsciiHex)),
            PlannedFilter::Ascii85 => (PdfStreamDataCursor::ascii85(maximum_output), Some(PdfStreamFilter::Ascii85)),
            PlannedFilter::RunLength => (PdfStreamDataCursor::run_length(maximum_output), Some(PdfStreamFilter::RunLength)),
            PlannedFilter::Dct => {
                if matches!(value(2, 3), Some(PdfObject::Array(parameters)) if parameters.len() != 1) { return Err(unsupported()); }
                let color_transform = match parameter {
                    None | Some(PdfObject::Null) => None,
                    Some(PdfObject::Dict(entries)) if entries.is_empty() => None,
                    Some(PdfObject::Dict(entries)) if entries.len() == 1 && entries[0].key == "ColorTransform" => {
                        Some(match entries[0].value.as_i64() {
                            Some(0) => 0,
                            Some(1) => 1,
                            _ => return Err(unsupported()),
                        })
                    }
                    _ => return Err(unsupported()),
                };
                (PdfStreamDataCursor::identity(maximum_output), Some(PdfStreamFilter::Dct { color_transform }))
            }
            PlannedFilter::Flate | PlannedFilter::Lzw => {
                if let Some(PdfObject::Dict(parameters)) = parameter {
                    if let Some(entry) = parameters.get(self.parameter_position) {
                        if let Some(slot) = ["Predictor", "Columns", "Colors", "BitsPerComponent", "EarlyChange"].iter().position(|&key| key == entry.key) {
                            if let Some(number) = entry.value.as_i64() {
                                let number = if slot == 0 || slot == 4 { number } else { number.max(1) };
                                self.parameters[slot].get_or_insert(number as u32);
                            }
                        }
                        self.parameter_position += 1;
                        return Ok(StreamDataStatus::More);
                    }
                }
                let predictor = match self.parameters[0].unwrap_or(1) {
                    1 => None,
                    predictor => Some(PdfPredictor {
                        predictor, columns: self.parameters[1].unwrap_or(1), colors: self.parameters[2].unwrap_or(1),
                        bits_per_component: self.parameters[3].unwrap_or(8),
                    }),
                };
                let maximum = self.maximum_inflated.min(maximum_output);
                let early_change = self.parameters[4].unwrap_or(1) != 0;
                let lzw = matches!(filter, PlannedFilter::Lzw);
                let decoder = match (lzw, predictor.as_ref()) {
                    (false, Some(p)) => PdfStreamDataCursor::flate_predicted(self.maximum_inflated, maximum_output, p.predictor, p.columns, p.colors, p.bits_per_component)?,
                    (true, Some(p)) => PdfStreamDataCursor::lzw_predicted(self.maximum_inflated, maximum_output, early_change, p.predictor, p.columns, p.colors, p.bits_per_component)?,
                    (false, None) => PdfStreamDataCursor::flate(maximum),
                    (true, None) => PdfStreamDataCursor::lzw(maximum, early_change),
                };
                let filter = if lzw { PdfStreamFilter::Lzw { predictor, early_change } } else { PdfStreamFilter::Flate { predictor } };
                (decoder, Some(filter))
            }
        };
        self.decoder = Some(decoder);
        if let Some(filter) = filter { self.filters.push(filter); }
        Ok(StreamDataStatus::More)
    }

    fn advance_handoff(&mut self, grant: StreamDataGrant) -> StreamDataReceipt {
        if grant.work == 0 { return empty(Ok(StreamDataStatus::More), 0); }
        if let Some(decoder) = self.decoder.as_mut() {
            if self.next_input.is_none() {
                let Some(bytes) = decoder.take_output() else { return empty(Err(invariant("PDF stream filter lost its staged output")), 1) };
                self.next_input = Some(bytes);
                return empty(Ok(StreamDataStatus::More), 1);
            }
            let mut receipt = decoder.close_step(grant);
            if matches!(receipt.status, Ok(StreamDataStatus::Closed)) {
                self.decoder = None;
                receipt.work = receipt.work.max(1);
                receipt.status = Ok(StreamDataStatus::More);
            }
            return receipt;
        }
        let bytes = self.input.as_ref().map_or(0, Vec::capacity);
        if bytes > grant.release_bytes { return empty(Ok(StreamDataStatus::NeedsRelease { bytes }), 0); }
        self.input = self.next_input.take();
        self.stage += 1;
        self.parameter_position = 0;
        self.parameters = [None; 5];
        self.finishing_stage = false;
        let mut receipt = empty(Ok(StreamDataStatus::More), 1);
        receipt.released_bytes = bytes;
        receipt
    }

    /// 📩️ Moves validated bytes and the typed pipeline, retaining internal retirement custody.
    pub fn take_output(&mut self) -> Option<DecodedStreamData> {
        let bytes = self.take_bytes()?;
        Some(DecodedStreamData { bytes, filters: std::mem::take(&mut self.filters) })
    }

    /// 📬️ Moves only the validated bytes, retaining filter metadata for controlled closure.
    pub fn take_bytes(&mut self) -> Option<Vec<u8>> {
        if self.cancelled || !self.complete || self.closing { return None; }
        let bytes = self.decoder.as_mut()?.take_output()?;
        self.closing = true;
        Some(bytes)
    }

    /// ♻️ Begins closing the stream at any stage.
    pub fn begin_close(&mut self) {
        self.closing = true;
        if let Some(decoder) = self.decoder.as_mut() { decoder.begin_close(); }
    }

    /// 🧹️ Advances output, intermediate backing, and filter retirement.
    pub fn close_step(&mut self, grant: StreamDataGrant) -> StreamDataReceipt {
        if !self.closing { return empty(Ok(StreamDataStatus::More), 0); }
        if self.decoder.is_none() && self.input.is_none() && self.next_input.is_none() && self.filters.capacity() == 0 {
            return empty(Ok(StreamDataStatus::Closed), 0);
        }
        if grant.work == 0 { return empty(Ok(StreamDataStatus::More), 0); }
        if let Some(decoder) = self.decoder.as_mut() {
            let mut receipt = decoder.close_step(grant);
            if matches!(receipt.status, Ok(StreamDataStatus::Closed)) {
                self.decoder = None;
                receipt.work = receipt.work.max(1);
                receipt.status = Ok(StreamDataStatus::More);
            }
            return receipt;
        }
        let input = self.input.as_ref().or(self.next_input.as_ref());
        let bytes = input.map_or_else(|| self.filters.capacity() * size_of::<PdfStreamFilter>(), Vec::capacity);
        if bytes > grant.release_bytes { return empty(Ok(StreamDataStatus::NeedsRelease { bytes }), 0); }
        let mut receipt = empty(Ok(StreamDataStatus::More), 1);
        receipt.released_bytes = bytes;
        if input.is_some() {
            if self.input.is_some() { self.input = None; } else { self.next_input = None; }
        } else {
            self.filters = Vec::new();
            receipt.status = Ok(StreamDataStatus::Closed);
        }
        receipt
    }
}

fn invariant(reason: &'static str) -> ValueError {
    ValueError::literal(ValueRefusalKind::InvariantViolated, reason)
}

enum PlannedFilter { Identity, AsciiHex, Ascii85, RunLength, Flate, Lzw, Dct }

fn planned_filter(declaration: Option<&PdfObject>, stage: usize) -> Result<PlannedFilter, ValueError> {
    use PlannedFilter as Filter;
    let name = match declaration {
        None | Some(PdfObject::Null) => return Ok(Filter::Identity),
        Some(PdfObject::Name(name)) => name.as_str(),
        Some(PdfObject::Array(filters)) if filters.is_empty() => return Ok(Filter::Identity),
        Some(PdfObject::Array(filters)) => filters.get(stage).and_then(PdfObject::as_name).ok_or_else(unsupported)?,
        _ => return Err(unsupported()),
    };
    match name {
        "ASCIIHexDecode" | "AHx" => Ok(Filter::AsciiHex),
        "ASCII85Decode" | "A85" => Ok(Filter::Ascii85),
        "RunLengthDecode" | "RL" => Ok(Filter::RunLength),
        "FlateDecode" | "Fl" => Ok(Filter::Flate),
        "LZWDecode" | "LZW" => Ok(Filter::Lzw),
        "DCTDecode" | "DCT" => Ok(Filter::Dct),
        _ => Err(unsupported()),
    }
}

fn unsupported() -> ValueError {
    ValueError::literal(ValueRefusalKind::UnsupportedOwner, "stream filter chain has no controlled decoder")
}

fn empty(status: Result<StreamDataStatus, ValueError>, work: usize) -> StreamDataReceipt {
    StreamDataReceipt { status, work, allocated_bytes: 0, released_bytes: 0 }
}

/// 🧰️ Drains the controlled decoder, retaining production recovery for other accepted streams.
pub fn decode_stream_admitted(dict: &[PdfDictEntry], raw: &[u8]) -> PResult<(Vec<u8>, Vec<PdfStreamFilter>)> {
    let mut cursor = PdfDecodedStreamCursor::new(usize::MAX, usize::MAX);
    let grant = StreamDataGrant { work: 4096, allocation_bytes: usize::MAX, release_bytes: usize::MAX };
    let output = loop {
        match cursor.advance(dict, raw, grant, false).status {
            Ok(StreamDataStatus::Complete) => break Ok(cursor.take_output()),
            Ok(StreamDataStatus::More) => {}
            Err(error) if error.kind == ValueRefusalKind::UnsupportedOwner
                || error.kind == ValueRefusalKind::InvalidValue && !matches!(cursor.filters.last(), Some(PdfStreamFilter::Lzw { .. })) => break Ok(None),
            Err(error) => break Err(PdfEngineError::Malformed(error.into_message())),
            Ok(_) => break malformed("unexpected controlled PDF stream status"),
        }
    };
    cursor.begin_close();
    while cursor.close_step(grant).status.ok() != Some(StreamDataStatus::Closed) {}
    match output? {
        Some(output) => Ok((output.bytes, output.filters)),
        None => decode_stream(dict, raw),
    }
}
