//! 🧱️ Fuelled assembly of one typed COS value.

use super::{ObjRef, PdfDictEntry, PdfObject};

#[path = "📦️buffers/🦀️.rs"]
mod buffers;
#[path = "🔡️tokens/🦀️.rs"]
mod tokens;

use buffers::Growing;
use tokens::{Reader, Token, MAX_SCALAR_BYTES};

const MAX_DEPTH: usize = 64;

/// 🎚️ Parser-owned limits checked before source-controlled allocations.
#[derive(Clone, Copy, Debug)]
pub struct CosLimits {
    pub depth: usize,
    pub container_items: usize,
    pub scalar_bytes: usize,
    pub string_bytes: usize,
}

impl Default for CosLimits {
    fn default() -> Self {
        Self { depth: MAX_DEPTH, container_items: 1_000_000, scalar_bytes: MAX_SCALAR_BYTES, string_bytes: 64 * 1024 * 1024 }
    }
}

/// 🎟️ Work, new capacity, and releases admitted for one call.
#[derive(Clone, Copy, Debug, Default)]
pub struct CosGrant {
    pub work: usize,
    pub allocation_bytes: usize,
    pub release_bytes: usize,
}

/// 🚦️ COS parser or retirement progress.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CosStatus {
    More,
    NeedsAllocation { bytes: usize },
    NeedsRelease { bytes: usize },
    Complete { end: usize },
    Cancelled,
    Closed,
}

/// 🧾️ Reports actual work and native capacities acquired or released in a turn.
#[derive(Debug, PartialEq, Eq)]
pub struct CosReceipt {
    pub status: Result<CosStatus, &'static str>,
    pub work: usize,
    pub allocated_bytes: usize,
    pub released_bytes: usize,
}

struct Turn {
    grant: CosGrant,
    work: usize,
    allocated_bytes: usize,
    released_bytes: usize,
    blocked: Option<CosStatus>,
}

impl Turn {
    fn new(grant: CosGrant) -> Self {
        Self { grant, work: 0, allocated_bytes: 0, released_bytes: 0, blocked: None }
    }

    fn allocation_fits(&mut self, bytes: usize) -> bool {
        if bytes <= self.grant.allocation_bytes - self.allocated_bytes { return true; }
        self.blocked = Some(CosStatus::NeedsAllocation { bytes });
        false
    }

    fn allocated(&mut self, bytes: usize) {
        self.allocated_bytes += bytes;
        assert!(self.allocated_bytes <= self.grant.allocation_bytes);
    }

    fn release(&mut self, bytes: usize) -> bool {
        if bytes > self.grant.release_bytes - self.released_bytes {
            self.blocked = Some(CosStatus::NeedsRelease { bytes });
            return false;
        }
        self.released_bytes += bytes;
        true
    }

    fn receipt(self, status: Result<CosStatus, &'static str>) -> CosReceipt {
        CosReceipt { status, work: self.work, allocated_bytes: self.allocated_bytes, released_bytes: self.released_bytes }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Container { Array, Dict }

enum Frame {
    Array(Growing<PdfObject>),
    Dict { entries: Growing<PdfDictEntry>, key: Option<String> },
}

impl Frame {
    fn open(kind: Container) -> Self {
        match kind {
            Container::Array => Self::Array(Growing::new()),
            Container::Dict => Self::Dict { entries: Growing::new(), key: None },
        }
    }

    fn expects_key(&self) -> bool { matches!(self, Self::Dict { key: None, .. }) }

    fn accepts_close(&self, kind: Container) -> bool {
        matches!((self, kind), (Self::Array(_), Container::Array) | (Self::Dict { key: None, .. }, Container::Dict))
    }

    fn finish(self) -> PdfObject {
        match self {
            Self::Array(items) => PdfObject::Array(items.into_vec()),
            Self::Dict { entries, .. } => PdfObject::Dict(entries.into_vec()),
        }
    }
}

#[derive(Clone, Copy)]
enum Lifecycle { Active, Complete(usize), Cancelled, Failed(&'static str), Closing, Closed }

/// 🧮️ Parses one native COS value without lending its partial object tree.
/// Cancellation and refusal keep all capacity resident until the caller drives retirement.
#[must_use]
pub struct PdfCosCursor {
    reader: Reader,
    limits: CosLimits,
    frames: [Option<Frame>; MAX_DEPTH],
    depth: usize,
    lifecycle: Lifecycle,
    token: Option<(Token, usize)>,
    integers: [Option<(i64, usize)>; 2],
    attachment: Option<(PdfObject, usize)>,
    value: Option<PdfObject>,
    retiring: Option<PdfObject>,
    retiring_scanner: bool,
}

impl PdfCosCursor {
    /// 🌱️ Starts on a stable caller-owned PDF source at a COS value offset.
    pub fn new(offset: usize, limits: CosLimits) -> Self {
        Self {
            reader: Reader::new(offset), limits, frames: std::array::from_fn(|_| None), depth: 0,
            lifecycle: Lifecycle::Active, token: None, integers: [None; 2], attachment: None,
            value: None, retiring: None, retiring_scanner: false,
        }
    }

    /// 📍 Latest inspected source position. The completion receipt carries the exact value end.
    pub fn inspected_position(&self) -> usize { self.reader.position }

    /// 🧭️ Advances bounded scanning or assembly. The source prefix must remain stable.
    pub fn advance(&mut self, source: &[u8], grant: CosGrant, cancelled: bool) -> CosReceipt {
        let mut turn = Turn::new(grant);
        if cancelled && matches!(self.lifecycle, Lifecycle::Active) { self.lifecycle = Lifecycle::Cancelled; }
        while let Lifecycle::Active = self.lifecycle {
            if turn.work == grant.work {
                return turn.receipt(Ok(CosStatus::More));
            }
            turn.work += 1;
            if let Err(error) = self.step(source, &mut turn) { self.lifecycle = Lifecycle::Failed(error); }
            if let Some(blocked) = turn.blocked { return turn.receipt(Ok(blocked)); }
        }
        let status = match self.lifecycle {
            Lifecycle::Complete(end) => Ok(CosStatus::Complete { end }),
            Lifecycle::Cancelled => Ok(CosStatus::Cancelled),
            Lifecycle::Failed(error) => Err(error),
            Lifecycle::Closing => Ok(CosStatus::More),
            Lifecycle::Closed => Ok(CosStatus::Closed),
            Lifecycle::Active => unreachable!(),
        };
        turn.receipt(status)
    }

    pub fn ready_value(&self) -> Option<&PdfObject> { self.value.as_ref().filter(|_| matches!(self.lifecycle, Lifecycle::Complete(_))) }

    /// 📬️ Hands the completed native value to its caller without cloning it.
    pub fn take_value(&mut self) -> Option<PdfObject> {
        if matches!(self.lifecycle, Lifecycle::Complete(_)) { self.value.take() } else { None }
    }

    /// 🔖️ Retires a parser-owned dictionary, including its nested values and original buffer.
    pub fn retire_dictionary(entries: Vec<PdfDictEntry>) -> Self { Self::retire_parsed_value(PdfObject::Dict(entries)) }

    /// 🪦️ Retires an original COS parser result under the same ownership grants as a partial value.
    pub fn retire_parsed_value(value: PdfObject) -> Self {
        let mut cursor = Self::new(0, CosLimits::default());
        cursor.retiring = Some(value);
        cursor.retiring_scanner = true;
        cursor.begin_close();
        cursor
    }

    /// 🗝️ Retires a detached parser-owned entry without allocating a temporary dictionary.
    pub fn retire_dictionary_entry(entry: PdfDictEntry) -> Self {
        let mut cursor = Self::new(0, CosLimits::default());
        cursor.push(Frame::Dict { entries: Growing::new(), key: Some(entry.key) });
        cursor.retiring = Some(entry.value);
        cursor.retiring_scanner = true;
        cursor.begin_close();
        cursor
    }

    /// ♻️ Starts bounded retirement of the cursor's remaining native ownership.
    pub fn begin_close(&mut self) {
        if !matches!(self.lifecycle, Lifecycle::Closing | Lifecycle::Closed) {
            self.lifecycle = Lifecycle::Closing;
        }
    }

    /// 🧹️ Retires partially assembled children and container backing under the caller's grant.
    pub fn close_step(&mut self, grant: CosGrant) -> CosReceipt {
        let mut turn = Turn::new(grant);
        while matches!(self.lifecycle, Lifecycle::Closing) && turn.work < grant.work {
            turn.work += 1;
            self.retire_step(&mut turn);
            if let Some(blocked) = turn.blocked { return turn.receipt(Ok(blocked)); }
        }
        let status = if self.terminal_is_empty() { CosStatus::Closed } else { CosStatus::More };
        turn.receipt(Ok(status))
    }

    /// 🧺️ True after the cursor has relinquished all retained allocations.
    pub fn terminal_is_empty(&self) -> bool { matches!(self.lifecycle, Lifecycle::Closed) }

    fn step(&mut self, source: &[u8], turn: &mut Turn) -> Result<(), &'static str> {
        if self.attachment.is_some() { return self.attach(turn); }
        if self.token.is_none() {
            if self.reader.ends_reference_lookahead(source) && self.promote_integer() {
                return Ok(());
            }
            let limit = self.limits.scalar_bytes.min(MAX_SCALAR_BYTES);
            if let Some(token) = self.reader.step(source, limit, self.limits.string_bytes, turn)? {
                self.token = Some((token, self.reader.position));
            }
            return Ok(());
        }
        let (token, end) = self.token.as_ref().expect("COS token");
        if let Token::Reference = token {
            if let [Some((num, _)), Some((gen, _))] = self.integers {
                if let (Ok(num), Ok(gen)) = (u32::try_from(num), u16::try_from(gen)) {
                    self.attachment = Some((PdfObject::Ref(ObjRef { num, gen }), *end));
                    self.integers = [None; 2];
                    self.token = None;
                    return Ok(());
                }
            }
        }
        if let Token::Value(PdfObject::Int(integer)) = token {
            if let Some(slot) = self.integers.iter_mut().find(|entry| entry.is_none()) {
                *slot = Some((*integer, *end));
                self.token = None;
                return Ok(());
            }
        }
        if self.promote_integer() { return Ok(()); }
        let (token, end) = self.token.take().expect("COS token");
        match token {
            Token::Value(value) => self.attachment = Some((value, end)),
            Token::Open(kind) => {
                if self.depth != 0 && self.frames[self.depth - 1].as_ref().is_some_and(Frame::expects_key) {
                    return Err("PDF COS dictionary key must be a name");
                }
                if self.depth == self.limits.depth.min(MAX_DEPTH) { return Err("PDF COS nesting limit exceeded"); }
                self.push(Frame::open(kind));
            }
            Token::Close(kind) => {
                let Some(last) = self.depth.checked_sub(1) else { return Err("unmatched PDF COS close delimiter"); };
                if !self.frames[last].as_ref().is_some_and(|frame| frame.accepts_close(kind)) {
                    return Err("mismatched or incomplete PDF COS container");
                }
                self.depth = last;
                self.attachment = Some((self.frames[last].take().expect("COS frame").finish(), end));
            }
            Token::Reference | Token::Unexpected => return Err("unexpected PDF COS keyword"),
            Token::End => return Err("unexpected end of PDF COS value"),
        }
        Ok(())
    }

    fn promote_integer(&mut self) -> bool {
        let Some((integer, end)) = self.integers[0].take() else { return false };
        self.attachment = Some((PdfObject::Int(integer), end));
        self.integers[0] = self.integers[1].take();
        true
    }

    fn attach(&mut self, turn: &mut Turn) -> Result<(), &'static str> {
        let Some(last) = self.depth.checked_sub(1) else {
            let (value, end) = self.attachment.take().expect("COS attachment");
            self.value = Some(value);
            self.lifecycle = Lifecycle::Complete(end);
            return Ok(());
        };
        match self.frames[last].as_mut().expect("COS frame") {
            Frame::Array(items) => {
                if items.prepare_push(self.limits.container_items, turn)? {
                    items.push(self.attachment.take().expect("COS array item").0);
                }
            }
            Frame::Dict { entries, key } => {
                if key.is_none() {
                    if !matches!(&self.attachment, Some((PdfObject::Name(_), _))) {
                        return Err("PDF COS dictionary key must be a name");
                    }
                    let (PdfObject::Name(name), _) = self.attachment.take().expect("COS dictionary key") else { unreachable!() };
                    *key = Some(name);
                } else if entries.prepare_push(self.limits.container_items, turn)? {
                    let value = self.attachment.take().expect("COS dictionary value").0;
                    entries.push(PdfDictEntry { key: key.take().expect("COS key"), value });
                }
            }
        }
        Ok(())
    }

    fn push(&mut self, frame: Frame) {
        self.frames[self.depth] = Some(frame);
        self.depth += 1;
    }

    fn retire_step(&mut self, turn: &mut Turn) {
        if let Some(value) = self.retiring.as_ref() {
            let bytes = match value {
                PdfObject::Array(_) | PdfObject::Dict(_) => {
                    let frame = match self.retiring.take().expect("COS retired container") {
                        PdfObject::Array(items) => Frame::Array(Growing::from_vec(items)),
                        PdfObject::Dict(entries) => Frame::Dict { entries: Growing::from_vec(entries), key: None },
                        _ => unreachable!(),
                    };
                    self.push(frame);
                    return;
                }
                PdfObject::Name(name) | PdfObject::Text(name) => name.capacity(),
                PdfObject::Str(bytes) => bytes.capacity(),
                PdfObject::Real(decimal) => decimal.coefficient.capacity(),
                PdfObject::Null | PdfObject::Bool(_) | PdfObject::Int(_) | PdfObject::Ref(_) | PdfObject::Date(_) => 0,
                _ => unreachable!("the COS retirement cursor separates stream backing"),
            };
            if turn.release(bytes) { self.retiring = None; }
            return;
        }
        if let Some((value, _)) = self.attachment.take() {
            self.retiring = Some(value);
            return;
        }
        if let Some((token, _)) = self.token.take() {
            if let Token::Value(value) = token { self.retiring = Some(value); }
            return;
        }
        if self.value.is_some() {
            self.retiring = self.value.take();
            return;
        }
        if !self.retiring_scanner {
            self.retiring_scanner = self.reader.retired(turn);
            return;
        }
        let Some(last) = self.depth.checked_sub(1) else {
            self.lifecycle = Lifecycle::Closed;
            return;
        };
        let consumed = match self.frames[last].as_mut().expect("COS retirement frame") {
            Frame::Array(items) => {
                if let Some(value) = items.pop() { self.retiring = Some(value); false } else { items.retire_empty(turn) }
            }
            Frame::Dict { entries, key } => {
                if let Some(name) = key.as_ref() {
                    if turn.release(name.capacity()) { *key = None; }
                    false
                } else if let Some(entry) = entries.last() {
                    if turn.release(entry.key.capacity()) {
                        self.retiring = Some(entries.pop().expect("COS retirement entry").value);
                    }
                    false
                } else {
                    entries.retire_empty(turn)
                }
            }
        };
        if consumed {
            self.frames[last] = None;
            self.depth = last;
        }
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
