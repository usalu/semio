//! 🎚️ Admits direct fixed-width ToUnicode maps under lexical and retained-output grants.

use super::{code_of, PdfObject, PdfToUnicode, PdfToUnicodeMapping};
use crate::standards::v1_7::subsets::base::modules::lexer::{
    ContentToken, ContentTokenCursor, ContentTokenEvent, CosGrant, CosReceipt, CosStatus, PdfCosCursor,
};
use std::mem::size_of;

const DESTINATION_UNITS: usize = 16;

#[derive(Clone, Copy, PartialEq, Eq)]
enum BlockKind { Codespace, Char, Range }

impl BlockKind {
    fn begin(word: &[u8]) -> Option<Self> {
        match word {
            b"begincodespacerange" => Some(Self::Codespace),
            b"beginbfchar" => Some(Self::Char),
            b"beginbfrange" => Some(Self::Range),
            _ => None,
        }
    }

    fn end(self) -> &'static [u8] {
        match self {
            Self::Codespace => b"endcodespacerange",
            Self::Char => b"endbfchar",
            Self::Range => b"endbfrange",
        }
    }
}

struct Block { kind: BlockKind, remaining: usize, field: u8, low: u32, high: u32 }
struct MappedText { low: u32, high: Option<u32>, units: [u16; DESTINATION_UNITS], count: usize, bytes: usize }

#[derive(Clone, Copy)]
enum Phase { Active, Complete, Cancelled, Deferred, Failed(&'static str), Closing, Closed }

/// 🧾️ Borrows a direct CMap source and admits one retained Unicode mapping at a time.
pub struct PdfDirectToUnicodeCursor {
    tokens: ContentTokenCursor,
    ready: bool,
    tokens_closed: bool,
    retiring: Option<PdfCosCursor>,
    block: Option<Block>,
    pending: Option<MappedText>,
    count: Option<usize>,
    space: Option<(u32, u32)>,
    width: usize,
    mappings: Vec<PdfToUnicodeMapping>,
    maximum_mappings: usize,
    phase: Phase,
}

impl PdfDirectToUnicodeCursor {
    /// 🌱 Starts with an empty result under the caller's mapping count ceiling.
    pub fn new(maximum_mappings: usize) -> Self {
        Self {
            tokens: ContentTokenCursor::new(0), ready: false, tokens_closed: false, retiring: None, block: None,
            pending: None, count: None, space: None, width: 0, mappings: Vec::new(), maximum_mappings, phase: Phase::Active,
        }
    }

    /// ⏭️ Distinguishes a safely outside-subset stream from malformed input.
    pub fn reference_required(&self) -> bool { matches!(self.phase, Phase::Deferred) }

    /// ▶️ Advances one source, lexical retirement, or Unicode output turn.
    pub fn advance(&mut self, source: &[u8], mut grant: CosGrant, cancelled: bool) -> CosReceipt {
        if cancelled && matches!(self.phase, Phase::Active | Phase::Complete) { self.phase = Phase::Cancelled; }
        match self.phase {
            Phase::Complete => return report(Ok(CosStatus::Complete { end: self.tokens.position() }), 0),
            Phase::Cancelled => return report(Ok(CosStatus::Cancelled), 0),
            Phase::Closed => return report(Ok(CosStatus::Closed), 0),
            Phase::Deferred => return report(Err("ToUnicode CMap needs the reference parser"), 0),
            Phase::Failed(reason) => return report(Err(reason), 0),
            _ if grant.work == 0 || matches!(self.phase, Phase::Closing) => return report(Ok(CosStatus::More), 0),
            Phase::Active | Phase::Closing => {}
        }
        grant.work = 1;
        if let Some(value) = self.retiring.as_mut() {
            let turn = value.close_step(grant);
            if turn.status == Ok(CosStatus::Closed) { self.retiring = None; }
            return CosReceipt { status: turn.status.map(|status| if status == CosStatus::Closed { CosStatus::More } else { status }), ..turn };
        }
        if self.pending.is_some() { return self.publish(grant); }
        if self.ready {
            self.ready = false;
            let result = match self.tokens.take().expect("admitted CMap token") {
                ContentToken::Value(value) => {
                    let result = self.value(&value);
                    self.retiring = Some(PdfCosCursor::retire_parsed_value(value));
                    result
                }
                ContentToken::Operator { start, end } => self.keyword(source.get(start..end).unwrap_or_default()),
                ContentToken::End => {
                    if self.block.is_some() || self.space.is_none() || self.mappings.is_empty() { return self.fail("incomplete ToUnicode CMap"); }
                    self.phase = Phase::Complete;
                    return report(Ok(CosStatus::Complete { end: self.tokens.position() }), 1);
                }
            };
            return match result {
                Ok(()) => report(Ok(CosStatus::More), 1),
                Err(None) => {
                    self.phase = Phase::Deferred;
                    report(Err("ToUnicode CMap needs the reference parser"), 1)
                }
                Err(Some(reason)) => self.fail(reason),
            };
        }
        let turn = self.tokens.advance(source, grant, false);
        let status = match turn.event {
            Ok(ContentTokenEvent::Ready) => { self.ready = true; Ok(CosStatus::More) }
            Ok(ContentTokenEvent::More) => Ok(CosStatus::More),
            Ok(ContentTokenEvent::NeedsAllocation { bytes }) => Ok(CosStatus::NeedsAllocation { bytes }),
            Ok(ContentTokenEvent::NeedsRelease { bytes }) => Ok(CosStatus::NeedsRelease { bytes }),
            Err(reason) => { self.phase = Phase::Failed(reason); Err(reason) }
            _ => { self.phase = Phase::Failed("unexpected CMap lexical state"); Err("unexpected CMap lexical state") }
        };
        CosReceipt { status, work: turn.work, allocated_bytes: turn.allocated_bytes, released_bytes: turn.released_bytes }
    }

    fn keyword(&mut self, word: &[u8]) -> Result<(), Option<&'static str>> {
        if let Some(block) = self.block.as_ref() {
            if word != block.kind.end() { return Err(None); }
            if block.remaining != 0 || block.field != 0 { return Err(Some("ToUnicode CMap section count does not match its body")); }
            self.block = None;
            return Ok(());
        }
        let count = self.count.take();
        if let Some(kind) = BlockKind::begin(word) {
            let count = count.filter(|count| *count > 0).ok_or(Some("missing ToUnicode CMap section count"))?;
            if kind == BlockKind::Codespace {
                if count != 1 || self.space.is_some() { return Err(None); }
            } else if self.space.is_none() || count > self.maximum_mappings.saturating_sub(self.mappings.len()) {
                return Err(None);
            }
            self.block = Some(Block { kind, remaining: count, field: 0, low: 0, high: 0 });
        } else if matches!(word, b"usecmap" | b"begincidchar" | b"begincidrange" | b"beginnotdefchar" | b"beginnotdefrange") {
            return Err(None);
        }
        Ok(())
    }

    fn value(&mut self, value: &PdfObject) -> Result<(), Option<&'static str>> {
        let Some(block) = self.block.as_mut() else {
            self.count = value.as_i64().and_then(|count| usize::try_from(count).ok());
            return Ok(());
        };
        if block.remaining == 0 { return Err(Some("too many ToUnicode CMap entries")); }
        let PdfObject::Str(bytes) = value else { return Err(None); };
        let width = if block.kind == BlockKind::Codespace && block.field == 0 {
            if !matches!(bytes.len(), 1 | 2) { return Err(None); }
            self.width = bytes.len();
            self.width
        } else {
            self.width
        };
        let last_code = match block.kind { BlockKind::Char => 0, _ => 1 };
        if block.field <= last_code {
            if bytes.len() != width { return Err(None); }
            let code = code_of(bytes);
            if self.space.is_some_and(|(low, high)| !(low..=high).contains(&code)) { return Err(None); }
            if block.field == 0 {
                block.low = code;
            } else {
                if code < block.low { return Err(Some("descending ToUnicode CMap range")); }
                block.high = code;
            }
            block.field += 1;
            if block.kind == BlockKind::Codespace && block.field == 2 {
                self.space = Some((block.low, block.high));
                block.remaining -= 1;
                block.field = 0;
            }
            return Ok(());
        }
        if bytes.is_empty() || !bytes.len().is_multiple_of(2) || bytes.len() > 2 * DESTINATION_UNITS { return Err(None); }
        let mut units = [0; DESTINATION_UNITS];
        for (unit, pair) in units.iter_mut().zip(bytes.chunks_exact(2)) {
            *unit = u16::from_be_bytes([pair[0], pair[1]]);
        }
        let count = bytes.len() / 2;
        let mut utf8 = 0;
        for character in char::decode_utf16(units[..count].iter().copied()) {
            utf8 += character.map_err(|_| None)?.len_utf8();
        }
        let high = (block.kind == BlockKind::Range).then_some(block.high);
        if let Some(high) = high {
            let start = u32::from(units[0]);
            let end = start + high - block.low;
            if count != 1 || end > 0xffff || (start <= 0xdfff && end >= 0xd800) { return Err(None); }
        }
        self.pending = Some(MappedText { low: block.low, high, units, count, bytes: utf8 });
        block.remaining -= 1;
        block.field = 0;
        Ok(())
    }

    fn publish(&mut self, grant: CosGrant) -> CosReceipt {
        if self.mappings.capacity() == 0 {
            let Some(bytes) = self.maximum_mappings.checked_mul(size_of::<PdfToUnicodeMapping>()) else {
                return self.fail("ToUnicode map backing exceeds the native limit");
            };
            if bytes > grant.allocation_bytes { return report(Ok(CosStatus::NeedsAllocation { bytes }), 0); }
            if self.mappings.try_reserve_exact(self.maximum_mappings).is_err() { return self.fail("unable to reserve ToUnicode mappings"); }
            return CosReceipt { allocated_bytes: self.mappings.capacity() * size_of::<PdfToUnicodeMapping>(), ..report(Ok(CosStatus::More), 1) };
        }
        let pending = self.pending.as_ref().expect("parsed ToUnicode mapping");
        if pending.bytes > grant.allocation_bytes {
            return report(Ok(CosStatus::NeedsAllocation { bytes: pending.bytes }), 0);
        }
        let pending = self.pending.take().expect("parsed ToUnicode mapping");
        let mut text = String::with_capacity(pending.bytes);
        text.extend(char::decode_utf16(pending.units[..pending.count].iter().copied()).map(|character| character.expect("measured UTF-16")));
        let allocated_bytes = text.capacity();
        self.mappings.push(match pending.high {
            Some(high) => PdfToUnicodeMapping::Range { low: pending.low, high, text },
            None => PdfToUnicodeMapping::Char { code: pending.low, text },
        });
        CosReceipt { allocated_bytes, ..report(Ok(CosStatus::More), 1) }
    }

    fn fail(&mut self, reason: &'static str) -> CosReceipt {
        self.phase = Phase::Failed(reason);
        report(Err(reason), 1)
    }

    /// 📦️ Moves the qualified document map after parsing the complete source.
    pub fn take_mapping(&mut self) -> Option<PdfToUnicode> {
        if !matches!(self.phase, Phase::Complete) { return None; }
        self.phase = Phase::Closing;
        self.tokens.begin_close();
        Some(PdfToUnicode { byte_width: self.width as u32, mappings: std::mem::take(&mut self.mappings) })
    }

    /// ♻️ Keeps unpublished text, lexical values and result backing for granted retirement.
    pub fn begin_close(&mut self) {
        if matches!(self.phase, Phase::Closed | Phase::Closing) { return; }
        self.tokens.begin_close();
        self.phase = Phase::Closing;
    }

    /// 🧹️ Retires one independent owner under explicit release credit.
    pub fn close_step(&mut self, mut grant: CosGrant) -> CosReceipt {
        if matches!(self.phase, Phase::Closed) { return report(Ok(CosStatus::Closed), 0); }
        if grant.work == 0 || !matches!(self.phase, Phase::Closing) { return report(Ok(CosStatus::More), 0); }
        grant.work = 1;
        if !self.tokens_closed {
            let turn = self.tokens.close_step(grant);
            let status = match turn.event {
                Ok(ContentTokenEvent::Closed) => { self.tokens_closed = true; Ok(CosStatus::More) }
                Ok(ContentTokenEvent::NeedsRelease { bytes }) => Ok(CosStatus::NeedsRelease { bytes }),
                Ok(_) => Ok(CosStatus::More),
                Err(reason) => Err(reason),
            };
            return CosReceipt { status, work: turn.work, allocated_bytes: turn.allocated_bytes, released_bytes: turn.released_bytes };
        }
        if let Some(value) = self.retiring.as_mut() {
            let turn = value.close_step(grant);
            if turn.status == Ok(CosStatus::Closed) { self.retiring = None; }
            return CosReceipt { status: turn.status.map(|status| if status == CosStatus::Closed { CosStatus::More } else { status }), ..turn };
        }
        if let Some(mapping) = self.mappings.last() {
            let released_bytes = match mapping {
                PdfToUnicodeMapping::Char { text, .. } | PdfToUnicodeMapping::Range { text, .. } => text.capacity(),
            };
            if released_bytes > grant.release_bytes { return report(Ok(CosStatus::NeedsRelease { bytes: released_bytes }), 0); }
            self.mappings.pop();
            return CosReceipt { released_bytes, ..report(Ok(CosStatus::More), 1) };
        }
        let released_bytes = self.mappings.capacity() * size_of::<PdfToUnicodeMapping>();
        if released_bytes > grant.release_bytes { return report(Ok(CosStatus::NeedsRelease { bytes: released_bytes }), 0); }
        self.mappings = Vec::new();
        self.phase = Phase::Closed;
        CosReceipt { released_bytes, ..report(Ok(CosStatus::Closed), 1) }
    }
}

fn report(status: Result<CosStatus, &'static str>, work: usize) -> CosReceipt {
    CosReceipt { status, work, allocated_bytes: 0, released_bytes: 0 }
}
