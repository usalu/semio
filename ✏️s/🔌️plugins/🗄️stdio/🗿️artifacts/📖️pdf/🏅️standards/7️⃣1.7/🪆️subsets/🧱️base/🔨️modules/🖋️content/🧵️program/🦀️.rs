//! 🧵️ Retains PDF content token, operator, and font state across bounded parse turns.

use super::{array_text_item, bounded_number, bounded_spelling, first_array, fixed_operator, named_operator, num, plain_text_byte, ContentToken, ContentTokenCursor, FontState, FontTable, NumericArray, PdfObject, PdfOp, PdfTextArrayItem, PdfTextString, SavedFont, TextShow};
use crate::standards::v1_7::subsets::base::modules::lexer::{ContentTokenEvent, ContentTokenReceipt, CosGrant, CosReceipt, CosStatus, PdfCosCursor};
use crate::standards::v1_7::subsets::base::modules::fonts::controlled_type1_text;
use super::{PdfBaseEncoding, PdfFixedCodeWidth};
use std::mem::{size_of, take};

const OPERAND_LIMIT: usize = 64;
const ATOMIC_NAME_BYTES: usize = 128;
const CONTROLLED_TEXT_BYTES: usize = 128;
const CONTROLLED_TEXT_ITEMS: usize = 16;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContentProgramEvent {
    More,
    NeedsAllocation { bytes: usize },
    NeedsRelease { bytes: usize },
    OperatorReady,
    Deferred { start: usize, end: usize },
    Complete,
    Cancelled,
    Closed,
}

#[derive(Debug)]
pub struct ContentProgramReceipt {
    pub event: Result<ContentProgramEvent, &'static str>,
    pub work: usize,
    pub allocated_bytes: usize,
    pub released_bytes: usize,
}

#[derive(Clone, Copy)]
enum Phase {
    Lex,
    TakeToken,
    Recover(&'static str),
    StoreOperand,
    Apply { start: usize, end: usize },
    ScanText { target: TextTarget, index: usize, bytes: usize },
    AllocateText { target: TextTarget, utf8_bytes: Option<usize> },
    CopyText { target: TextTarget, index: usize },
    AllocateTextArray,
    TextArray { index: usize },
    AllocateNumbers { target: NumericArray },
    CopyNumbers { target: NumericArray, index: usize },
    Drain { complete: bool },
    Ready,
    Deferred { start: usize, end: usize },
    Complete,
    Failed(&'static str),
    Cancelled,
    Closing,
    Closed,
}

#[derive(Clone, Copy)]
enum TextTarget {
    Show(TextShow),
    Array { index: usize },
}

impl TextTarget {
    fn bytes(self, operands: &[PdfObject]) -> &[u8] {
        match self {
            Self::Show(show) => show.bytes(operands),
            Self::Array { index } => first_array(operands).get(index).and_then(PdfObject::as_str_bytes).unwrap_or(&[]),
        }
    }
}

/// 🧭️ Applies complete operators from one unchanged content stream and lends one output at a time.
pub struct ContentProgramCursor {
    phase: Phase,
    tokens: ContentTokenCursor,
    tokens_closed: bool,
    operands: Vec<PdfObject>,
    operand: Option<PdfObject>,
    retiring: Option<PdfCosCursor>,
    fonts: FontState,
    text: Option<PdfTextString>,
    encoding: Option<PdfBaseEncoding>,
    code_width: Option<PdfFixedCodeWidth>,
    text_array: Vec<PdfTextArrayItem>,
    numbers: Vec<f64>,
    pending: Option<PdfOp>,
    output_bytes: usize,
    operator_span: (usize, usize),
}

impl ContentProgramCursor {
    /// 🚀️ Starts at an authored token boundary. Each advance borrows the same content and font table.
    pub fn new(position: usize) -> Self {
        Self {
            phase: Phase::Lex,
            tokens: ContentTokenCursor::new(position),
            tokens_closed: false,
            operands: Vec::new(),
            operand: None,
            retiring: None,
            fonts: FontState::default(),
            text: None,
            encoding: None,
            code_width: None,
            text_array: Vec::new(),
            numbers: Vec::new(),
            pending: None,
            output_bytes: 0,
            operator_span: (0, 0),
        }
    }

    /// 🔤️ Returns the authored font selected by the operators already applied.
    pub fn selected_font(&self) -> Option<&str> { self.fonts.current.as_deref() }

    /// 👁️ Borrows the ready native operator before accepting its backing.
    pub fn peek_operator(&self) -> Option<&PdfOp> {
        if matches!(self.phase, Phase::Ready) { self.pending.as_ref() } else { None }
    }

    /// 🫴️ Transfers the typed operator and its payload custody from a ready cursor.
    pub fn take_operator(&mut self) -> Option<PdfOp> {
        if !matches!(self.phase, Phase::Ready) { return None; }
        self.phase = Phase::Lex;
        self.output_bytes = 0;
        self.pending.take()
    }

    /// ▶️ Advances lexing, demarcation, semantic application, or operand retirement.
    pub fn advance(&mut self, source: &[u8], fonts: &dyn FontTable, grant: CosGrant, cancelled: bool) -> ContentProgramReceipt {
        use ContentProgramEvent as Event;
        if cancelled && !matches!(self.phase, Phase::Closing | Phase::Closed | Phase::Complete) {
            self.phase = Phase::Cancelled;
        }
        match self.phase {
            Phase::Ready => receipt(Event::OperatorReady),
            Phase::Deferred { start, end } => receipt(Event::Deferred { start, end }),
            Phase::Failed(reason) => failure(reason),
            Phase::Complete => receipt(Event::Complete),
            Phase::Cancelled => receipt(Event::Cancelled),
            Phase::Closing => self.close_step(grant),
            Phase::Closed => receipt(Event::Closed),
            _ if grant.work == 0 => receipt(Event::More),
            Phase::Lex if self.retiring.is_some() => self.retire_value(grant),
            Phase::Lex => {
                let child = self.tokens.advance(source, grant, false);
                match child.event {
                    Ok(ContentTokenEvent::Ready) => self.phase = Phase::TakeToken,
                    Err(reason) => self.phase = Phase::Recover(reason),
                    _ => {}
                }
                token_receipt(child)
            }
            Phase::TakeToken => {
                self.phase = match self.tokens.take() {
                    Some(ContentToken::Value(value)) => {
                        self.operand = Some(value);
                        Phase::StoreOperand
                    }
                    Some(ContentToken::Operator { start, end }) => Phase::Apply { start, end },
                    Some(ContentToken::End) => Phase::Drain { complete: true },
                    None => Phase::Failed("ready PDF content token is unavailable"),
                };
                worked(0, 0)
            }
            Phase::Recover(reason) => {
                self.phase = if self.tokens.recover() { Phase::Lex } else { Phase::Failed(reason) };
                worked(0, 0)
            }
            Phase::StoreOperand => self.store_operand(grant),
            Phase::Apply { start, end } => self.apply(source, fonts, grant, start, end),
            Phase::ScanText { target, index, bytes } => {
                self.phase = match target.bytes(&self.operands).get(index).copied() {
                    Some(byte) => match text_byte_len(self.encoding, byte) {
                        Some(len) => Phase::ScanText { target, index: index + 1, bytes: bytes + len },
                        None => Phase::AllocateText { target, utf8_bytes: None },
                    },
                    None => Phase::AllocateText { target, utf8_bytes: Some(bytes) },
                };
                worked(0, 0)
            }
            Phase::AllocateText { target, utf8_bytes } => self.allocate_text(target, utf8_bytes, grant),
            Phase::CopyText { target, index } => self.copy_text(target, index),
            Phase::AllocateTextArray => self.allocate_text_array(grant),
            Phase::TextArray { index } => self.text_array_item(index),
            Phase::AllocateNumbers { target } => self.allocate_numbers(target, grant),
            Phase::CopyNumbers { target, index } => self.copy_numbers(target, index, grant),
            Phase::Drain { .. } if self.retiring.is_some() => self.retire_value(grant),
            Phase::Drain { complete } => {
                if let Some(value) = self.operands.pop() {
                    self.retiring = Some(PdfCosCursor::retire_parsed_value(value));
                    return worked(0, 0);
                }
                self.phase = if complete { Phase::Complete } else { Phase::Ready };
                result(if complete { Event::Complete } else { Event::OperatorReady }, 1, 0, 0)
            }
        }
    }

    /// ♻️ Starts retirement on success, refusal, cancellation, or failure.
    pub fn begin_close(&mut self) {
        if !matches!(self.phase, Phase::Closing | Phase::Closed) {
            self.tokens.begin_close();
            self.phase = Phase::Closing;
        }
    }

    /// 🧺️ Releases remaining operand, semantic, and font custody within the supplied grant.
    pub fn close_step(&mut self, grant: CosGrant) -> ContentProgramReceipt {
        use ContentProgramEvent as Event;
        match self.phase {
            Phase::Closed => return receipt(Event::Closed),
            Phase::Closing if grant.work > 0 => {}
            _ => return receipt(Event::More),
        }
        if !self.tokens_closed {
            let mut child = self.tokens.close_step(grant);
            if child.event == Ok(ContentTokenEvent::Closed) {
                self.tokens_closed = true;
                child.work = child.work.max(1);
            }
            return token_receipt(child);
        }
        if self.retiring.is_some() { return self.retire_value(grant); }
        if let Some(value) = self.operand.take().or_else(|| self.operands.pop()) {
            self.retiring = Some(PdfCosCursor::retire_parsed_value(value));
            return worked(0, 0);
        }
        if matches!(self.pending, Some(PdfOp::ShowTextArray { .. })) {
            let Some(PdfOp::ShowTextArray { items }) = self.pending.take() else { unreachable!() };
            self.text_array = items;
            return worked(0, 0);
        }
        if let Some(text) = &self.text {
            let bytes = match text {
                PdfTextString::Text { text } => text.capacity(),
                PdfTextString::Codes { codes } => codes.capacity() * size_of::<u32>(),
            };
            if bytes > grant.release_bytes { return receipt(Event::NeedsRelease { bytes }); }
            self.text = None;
            self.output_bytes -= bytes;
            return worked(0, bytes);
        }
        if let Some(item) = self.text_array.last() {
            let bytes = match item {
                PdfTextArrayItem::Text { text } => text.capacity(),
                PdfTextArrayItem::Codes { codes } => codes.capacity() * size_of::<u32>(),
                PdfTextArrayItem::Adjust { .. } => 0,
            };
            if bytes > grant.release_bytes { return receipt(Event::NeedsRelease { bytes }); }
            self.text_array.pop();
            self.output_bytes -= bytes;
            return worked(0, bytes);
        }
        if self.pending.is_some() || self.numbers.capacity() > 0 || self.text_array.capacity() > 0 {
            let bytes = self.output_bytes;
            if bytes > grant.release_bytes { return receipt(Event::NeedsRelease { bytes }); }
            self.pending = None;
            self.text_array = Vec::new();
            self.numbers = Vec::new();
            self.output_bytes = 0;
            return worked(0, bytes);
        }
        if let Some(font) = &self.fonts.current {
            let bytes = font.capacity();
            if bytes > grant.release_bytes { return receipt(Event::NeedsRelease { bytes }); }
            self.fonts.current = None;
            return worked(0, bytes);
        }
        if let Some(frame) = &self.fonts.saved {
            let bytes = size_of::<SavedFont>() + string_capacity(frame.name.as_ref());
            if bytes > grant.release_bytes { return receipt(Event::NeedsRelease { bytes }); }
            let mut frame = self.fonts.saved.take().expect("ready PDF graphics font frame");
            self.fonts.saved = frame.next.take();
            return worked(0, bytes);
        }
        let bytes = self.operands.capacity() * size_of::<PdfObject>();
        if bytes > grant.release_bytes { return receipt(Event::NeedsRelease { bytes }); }
        self.operands = Vec::new();
        self.phase = Phase::Closed;
        result(Event::Closed, 1, 0, bytes)
    }

    fn store_operand(&mut self, grant: CosGrant) -> ContentProgramReceipt {
        let bytes = if self.operands.capacity() == 0 { OPERAND_LIMIT * size_of::<PdfObject>() } else { 0 };
        if bytes > grant.allocation_bytes { return receipt(ContentProgramEvent::NeedsAllocation { bytes }); }
        if bytes > 0 { self.operands = Vec::with_capacity(OPERAND_LIMIT); }
        let value = self.operand.take().expect("ready PDF content operand");
        if self.operands.len() < OPERAND_LIMIT {
            self.operands.push(value);
        } else {
            self.retiring = Some(PdfCosCursor::retire_parsed_value(value));
        }
        self.phase = Phase::Lex;
        worked(bytes, 0)
    }

    fn retire_value(&mut self, grant: CosGrant) -> ContentProgramReceipt {
        let child = self.retiring.as_mut().expect("retiring PDF content operand").close_step(grant);
        if child.status == Ok(CosStatus::Closed) { self.retiring = None; }
        cos_receipt(child)
    }

    fn apply(&mut self, source: &[u8], fonts: &dyn FontTable, grant: CosGrant, start: usize, end: usize) -> ContentProgramReceipt {
        let Some(bytes) = source.get(start..end) else {
            self.phase = Phase::Failed("PDF content source changed while applying operator");
            return failure("PDF content source changed while applying operator");
        };
        if bytes.len() > 3 { return self.defer(start, end); }
        let Ok(word) = std::str::from_utf8(bytes) else { return self.defer(start, end); };
        if self.operands.iter().any(|value| !bounded_spelling(value)) { return self.defer(start, end); }
        self.operator_span = (start, end);
        if let Some(op) = fixed_operator(word, &self.operands) { return self.finish(op, 0, 0, 0); }
        if let Some(show) = TextShow::new(word, &self.operands) {
            if !self.select_text_encoding(fonts) { return self.defer(start, end); }
            return self.start_text(TextTarget::Show(show));
        }
        if word == "TJ" {
            if !self.select_text_encoding(fonts) { return self.defer(start, end); }
            if (self.encoding.is_some() || self.code_width.is_some())
                && first_array(&self.operands).len() > CONTROLLED_TEXT_ITEMS { return self.defer(start, end); }
            self.phase = Phase::AllocateTextArray;
            return worked(0, 0);
        }
        if let Some(target) = NumericArray::new(word) {
            if target.pattern(&self.operands).is_some_and(|name| name.len() > ATOMIC_NAME_BYTES) { return self.defer(start, end); }
            self.phase = Phase::AllocateNumbers { target };
            return worked(0, 0);
        }
        let current = self.fonts.current.as_ref();
        let (allocation, release, output) = match word {
            "q" => (size_of::<SavedFont>() + current.map_or(0, String::len), 0, 0),
            "Q" => (0, self.fonts.saved.as_ref().map_or(0, |_| size_of::<SavedFont>() + string_capacity(current)), 0),
            "Tf" => {
                let len = self.operands.first().and_then(PdfObject::as_name).map_or(0, str::len);
                if len > ATOMIC_NAME_BYTES { return self.defer(start, end); }
                (2 * len, string_capacity(current), len)
            }
            _ => return self.apply_named(word, grant, start, end),
        };
        if allocation > grant.allocation_bytes { return receipt(ContentProgramEvent::NeedsAllocation { bytes: allocation }); }
        if release > grant.release_bytes { return receipt(ContentProgramEvent::NeedsRelease { bytes: release }); }
        let op = match word {
            "q" => self.fonts.save(),
            "Q" => self.fonts.restore(),
            "Tf" => self.fonts.select(&self.operands),
            _ => unreachable!("retained PDF font operator"),
        };
        self.finish(op, output, allocation, release)
    }

    fn apply_named(&mut self, word: &str, grant: CosGrant, start: usize, end: usize) -> ContentProgramReceipt {
        let source = self.operands.first().and_then(PdfObject::as_name).unwrap_or_default();
        let bytes = source.len();
        if named_operator(word, String::new).is_none() || bytes > ATOMIC_NAME_BYTES { return self.defer(start, end); }
        if bytes > grant.allocation_bytes { return receipt(ContentProgramEvent::NeedsAllocation { bytes }); }
        let op = named_operator(word, || source.to_owned()).expect("named PDF content operator");
        self.finish(op, bytes, bytes, 0)
    }

    fn defer_unbounded_number(&mut self) -> ContentProgramReceipt {
        let (start, end) = self.operator_span;
        self.defer(start, end)
    }

    fn defer(&mut self, start: usize, end: usize) -> ContentProgramReceipt {
        self.phase = Phase::Deferred { start, end };
        result(ContentProgramEvent::Deferred { start, end }, 1, 0, 0)
    }

    fn select_text_encoding(&mut self, fonts: &dyn FontTable) -> bool {
        self.encoding = self.selected_font().and_then(|name| fonts.controlled_type1(name));
        self.code_width = self.selected_font().and_then(|name| fonts.controlled_code_width(name));
        self.encoding.is_some() || self.code_width.is_some() || self.fonts.codec(fonts).is_none()
    }

    fn native_width(&self) -> usize { self.code_width.map_or(1, |width| width as usize) }

    fn start_text(&mut self, target: TextTarget) -> ContentProgramReceipt {
        let length = target.bytes(&self.operands).len();
        if (self.encoding.is_some() || self.code_width.is_some()) && length > CONTROLLED_TEXT_BYTES {
            let (start, end) = self.operator_span;
            return self.defer(start, end);
        }
        self.phase = if self.code_width.is_some() {
            if !length.is_multiple_of(self.native_width()) {
                self.phase = Phase::Failed("PDF text ends inside its admitted fixed-width codespace");
                return failure("PDF text ends inside its admitted fixed-width codespace");
            }
            Phase::AllocateText { target, utf8_bytes: None }
        } else {
            Phase::ScanText { target, index: 0, bytes: 0 }
        };
        worked(0, 0)
    }

    fn allocate_text(&mut self, target: TextTarget, utf8_bytes: Option<usize>, grant: CosGrant) -> ContentProgramReceipt {
        let len = target.bytes(&self.operands).len() / self.native_width();
        let Some(bytes) = utf8_bytes.or_else(|| len.checked_mul(size_of::<u32>())) else {
            self.phase = Phase::Failed("PDF content text capacity overflows");
            return failure("PDF content text capacity overflows");
        };
        if bytes > grant.allocation_bytes { return receipt(ContentProgramEvent::NeedsAllocation { bytes }); }
        self.text = Some(if utf8_bytes.is_some() { PdfTextString::Text { text: String::with_capacity(bytes) } } else { PdfTextString::Codes { codes: Vec::with_capacity(len) } });
        self.output_bytes += bytes;
        self.phase = Phase::CopyText { target, index: 0 };
        worked(bytes, 0)
    }

    fn copy_text(&mut self, target: TextTarget, index: usize) -> ContentProgramReceipt {
        let width = self.native_width();
        let source = target.bytes(&self.operands);
        if let Some(byte) = source.get(index).copied() {
            match self.text.as_mut().expect("allocated PDF semantic text") {
                PdfTextString::Text { text } => match self.encoding {
                    Some(encoding) => text.push_str(controlled_type1_text(encoding, byte).expect("scanned Type1 text")),
                    None => text.push(char::from(byte)),
                },
                PdfTextString::Codes { codes } => {
                    let code = if width == 2 { u32::from(u16::from_be_bytes([byte, source[index + 1]])) } else { u32::from(byte) };
                    codes.push(code);
                }
            }
            self.phase = Phase::CopyText { target, index: index + width };
            return worked(0, 0);
        }
        let text = self.text.take().expect("complete PDF semantic text");
        match target {
            TextTarget::Show(show) => self.finish(show.finish(text), self.output_bytes, 0, 0),
            TextTarget::Array { index } => {
                self.text_array.push(array_text_item(text));
                self.phase = Phase::TextArray { index: index + 1 };
                worked(0, 0)
            }
        }
    }

    fn allocate_text_array(&mut self, grant: CosGrant) -> ContentProgramReceipt {
        let count = first_array(&self.operands).len();
        let bytes = count * size_of::<PdfTextArrayItem>();
        if bytes > grant.allocation_bytes { return receipt(ContentProgramEvent::NeedsAllocation { bytes }); }
        self.text_array = Vec::with_capacity(count);
        self.output_bytes = bytes;
        self.phase = Phase::TextArray { index: 0 };
        worked(bytes, 0)
    }

    fn text_array_item(&mut self, index: usize) -> ContentProgramReceipt {
        match first_array(&self.operands).get(index) {
            Some(PdfObject::Str(_)) => {
                return self.start_text(TextTarget::Array { index });
            }
            Some(value) => {
                let Ok(amount) = bounded_number(value) else { return self.defer_unbounded_number(); };
                if let Some(amount) = amount { self.text_array.push(PdfTextArrayItem::Adjust { amount }); }
                self.phase = Phase::TextArray { index: index + 1 };
            }
            None => {
                let items = take(&mut self.text_array);
                return self.finish(PdfOp::ShowTextArray { items }, self.output_bytes, 0, 0);
            }
        }
        worked(0, 0)
    }

    fn allocate_numbers(&mut self, target: NumericArray, grant: CosGrant) -> ContentProgramReceipt {
        let count = target.values(&self.operands).len();
        let bytes = count * size_of::<f64>();
        if bytes > grant.allocation_bytes { return receipt(ContentProgramEvent::NeedsAllocation { bytes }); }
        self.numbers = Vec::with_capacity(count);
        self.output_bytes = bytes;
        self.phase = Phase::CopyNumbers { target, index: 0 };
        worked(bytes, 0)
    }

    fn copy_numbers(&mut self, target: NumericArray, index: usize, grant: CosGrant) -> ContentProgramReceipt {
        if let Some(value) = target.values(&self.operands).get(index) {
            let Ok(number) = bounded_number(value) else { return self.defer_unbounded_number(); };
            if let Some(number) = number { self.numbers.push(number); }
            self.phase = Phase::CopyNumbers { target, index: index + 1 };
            return worked(0, 0);
        }
        let bytes = target.pattern(&self.operands).map_or(0, str::len);
        if bytes > grant.allocation_bytes { return receipt(ContentProgramEvent::NeedsAllocation { bytes }); }
        let values = take(&mut self.numbers);
        let op = target.finish(values, &self.operands);
        self.finish(op, self.output_bytes + bytes, bytes, 0)
    }

    fn finish(&mut self, op: PdfOp, output_bytes: usize, allocated_bytes: usize, released_bytes: usize) -> ContentProgramReceipt {
        self.pending = Some(op);
        self.output_bytes = output_bytes;
        self.phase = Phase::Drain { complete: false };
        worked(allocated_bytes, released_bytes)
    }
}

fn text_byte_len(encoding: Option<PdfBaseEncoding>, byte: u8) -> Option<usize> {
    match encoding {
        Some(encoding) => controlled_type1_text(encoding, byte).map(str::len),
        None => plain_text_byte(byte).then_some(1),
    }
}

fn string_capacity(value: Option<&String>) -> usize { value.map_or(0, String::capacity) }

fn result(event: ContentProgramEvent, work: usize, allocated_bytes: usize, released_bytes: usize) -> ContentProgramReceipt {
    ContentProgramReceipt { event: Ok(event), work, allocated_bytes, released_bytes }
}

fn receipt(event: ContentProgramEvent) -> ContentProgramReceipt { result(event, 0, 0, 0) }

fn worked(allocated: usize, released: usize) -> ContentProgramReceipt { result(ContentProgramEvent::More, 1, allocated, released) }

fn failure(reason: &'static str) -> ContentProgramReceipt {
    ContentProgramReceipt { event: Err(reason), work: 0, allocated_bytes: 0, released_bytes: 0 }
}

fn token_receipt(child: ContentTokenReceipt) -> ContentProgramReceipt {
    let event = match child.event {
        Ok(ContentTokenEvent::NeedsAllocation { bytes }) => ContentProgramEvent::NeedsAllocation { bytes },
        Ok(ContentTokenEvent::NeedsRelease { bytes }) => ContentProgramEvent::NeedsRelease { bytes },
        _ => ContentProgramEvent::More,
    };
    result(event, child.work, child.allocated_bytes, child.released_bytes)
}

fn cos_receipt(child: CosReceipt) -> ContentProgramReceipt {
    let mut turn = result(ContentProgramEvent::More, child.work, child.allocated_bytes, child.released_bytes);
    turn.event = child.status.map(|status| match status {
        CosStatus::NeedsRelease { bytes } => ContentProgramEvent::NeedsRelease { bytes },
        _ => ContentProgramEvent::More,
    });
    turn
}
