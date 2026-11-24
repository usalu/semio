//! 📦️ Bounded object stream index pairs and indexed COS members.

use semio_s_artifact_stdio_pdf_lexer::{is_ws, CosGrant, CosLimits, CosReceipt, CosStatus, PdfCosCursor};
use semio_framework_value::ByteReadSource;
use semio_s_artifact_stdio_pdf_snapshot_2::standards::v1_7::subsets::base::schema::snapshot::PdfObject;

/// 🧾 The next object stream header event.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ObjectStreamHeaderEvent {
    More,
    Member { number: u32, offset: usize },
    Complete,
    Cancelled,
}

/// 🪙 Accounts for source inspection even when an index pair is invalid.
#[derive(Debug, PartialEq, Eq)]
pub struct ObjectStreamHeaderReceipt {
    pub event: Result<ObjectStreamHeaderEvent, &'static str>,
    pub work: usize,
}

#[derive(Clone, Copy)]
enum HeaderPhase { Between, Comment, Number { value: usize, digits: bool }, Cancelled, Failed(&'static str) }

/// 🧭 Walks the integer pairs before /First without allocating a member table.
pub struct ObjectStreamHeaderCursor {
    position: usize,
    first: usize,
    remaining: usize,
    phase: HeaderPhase,
    number: Option<u32>,
    last_offset: Option<usize>,
}

impl ObjectStreamHeaderCursor {
    /// 🌱 Binds the declared member count and first body offset.
    pub fn new(members: usize, first: usize) -> Self {
        Self { position: 0, first, remaining: members, phase: HeaderPhase::Between, number: None, last_offset: None }
    }

    /// ▶️ Emits at most one member index using the admitted header byte work.
    pub fn advance(&mut self, decoded: &(impl ByteReadSource + ?Sized), grant: usize, cancelled: bool) -> ObjectStreamHeaderReceipt {
        if cancelled && self.remaining != 0 && matches!(self.phase, HeaderPhase::Between | HeaderPhase::Comment | HeaderPhase::Number { .. }) {
            self.phase = HeaderPhase::Cancelled;
        }
        let mut work = 0;
        let event = loop {
            match self.phase {
                HeaderPhase::Cancelled => break Ok(ObjectStreamHeaderEvent::Cancelled),
                HeaderPhase::Failed(reason) => break Err(reason),
                _ if self.remaining == 0 => break Ok(ObjectStreamHeaderEvent::Complete),
                _ if work == grant => break Ok(ObjectStreamHeaderEvent::More),
                _ => {}
            }
            work += 1;
            let result = self.advance_byte(decoded);
            match result {
                Ok(None) => {}
                Ok(Some(event)) => break Ok(event),
                Err(reason) => { self.phase = HeaderPhase::Failed(reason); break Err(reason); }
            }
        };
        ObjectStreamHeaderReceipt { event, work }
    }

    fn advance_byte(&mut self, decoded: &(impl ByteReadSource + ?Sized)) -> Result<Option<ObjectStreamHeaderEvent>, &'static str> {
        if self.first > decoded.byte_len() { return Err("object stream /First exceeds its decoded bytes"); }
        let byte = if self.position < self.first { decoded.byte_at(self.position) } else { None };
        match (self.phase, byte) {
            (HeaderPhase::Between, Some(b'%')) => { self.phase = HeaderPhase::Comment; }
            (HeaderPhase::Between, Some(b)) if is_ws(b) => {}
            (HeaderPhase::Between, Some(b'+')) => { self.phase = HeaderPhase::Number { value: 0, digits: false }; }
            (HeaderPhase::Between, Some(b @ b'0'..=b'9')) => {
                self.phase = HeaderPhase::Number { value: usize::from(b - b'0'), digits: true };
            }
            (HeaderPhase::Comment, Some(b'\r' | b'\n')) => { self.phase = HeaderPhase::Between; }
            (HeaderPhase::Comment, Some(_)) => {}
            (HeaderPhase::Number { value, .. }, Some(b @ b'0'..=b'9')) => {
                let value = value.checked_mul(10).and_then(|number| number.checked_add(usize::from(b - b'0'))).ok_or("object stream header integer overflow")?;
                self.phase = HeaderPhase::Number { value, digits: true };
            }
            (HeaderPhase::Number { value, digits: true }, next) if next.is_none_or(|b| is_ws(b) || b == b'%') => {
                self.phase = HeaderPhase::Between;
                return self.finish_integer(value, decoded.byte_len());
            }
            (_, None) => return Err("incomplete object stream header"),
            _ => return Err("invalid object stream header integer"),
        }
        self.position += 1;
        Ok(None)
    }

    fn finish_integer(&mut self, value: usize, body_end: usize) -> Result<Option<ObjectStreamHeaderEvent>, &'static str> {
        let Some(number) = self.number.take() else {
            let number = u32::try_from(value).map_err(|_| "invalid object stream number")?;
            if number == 0 { return Err("invalid object stream number"); }
            self.number = Some(number);
            return Ok(None);
        };
        let offset = self.first.checked_add(value).filter(|&offset| offset < body_end).ok_or("object stream member offset exceeds its decoded bytes")?;
        if self.last_offset.is_some_and(|previous| previous >= offset) { return Err("object stream offsets are not strictly increasing"); }
        self.last_offset = Some(offset);
        self.remaining -= 1;
        Ok(Some(ObjectStreamHeaderEvent::Member { number, offset }))
    }
}

enum MemberPhase { Value, Tail { position: usize, comment: bool }, Complete, Failed(&'static str), Cancelled, Closing }

/// 🔍 Decodes one COS member without letting it read or absorb another member.
#[must_use]
pub struct ObjectStreamMemberCursor {
    value: PdfCosCursor,
    end: usize,
    phase: MemberPhase,
}

impl ObjectStreamMemberCursor {
    /// 🌱 Opens the selected member within the validated object stream range.
    pub fn new(start: usize, end: usize, limits: CosLimits) -> Self {
        let phase = if start < end { MemberPhase::Value } else { MemberPhase::Failed("empty object stream member range") };
        Self { value: PdfCosCursor::new(start, limits), end, phase }
    }

    /// ▶️ Drives the typed COS member and its bounded trailing whitespace/comments.
    pub fn advance(&mut self, decoded: &(impl ByteReadSource + ?Sized), grant: CosGrant, cancelled: bool) -> CosReceipt {
        if cancelled && matches!(self.phase, MemberPhase::Value | MemberPhase::Tail { .. }) { self.phase = MemberPhase::Cancelled; }
        let mut receipt = CosReceipt { status: Ok(CosStatus::More), work: 0, allocated_bytes: 0, released_bytes: 0 };
        if !matches!(self.phase, MemberPhase::Value | MemberPhase::Tail { .. }) || grant.work == 0 {
            receipt.status = match self.phase {
                MemberPhase::Complete => Ok(CosStatus::Complete { end: self.end }),
                MemberPhase::Failed(reason) => Err(reason),
                MemberPhase::Cancelled => Ok(CosStatus::Cancelled),
                _ => Ok(CosStatus::More),
            };
            return receipt;
        }
        let end = self.end;
        if end > decoded.byte_len() {
            self.phase = MemberPhase::Failed("object stream member exceeds its decoded bytes");
            receipt.status = Err("object stream member exceeds its decoded bytes");
            receipt.work = 1;
            return receipt;
        }
        let source = |position: usize| if position < end { decoded.byte_at(position) } else { None };
        receipt.status = match self.phase {
            MemberPhase::Value => {
                let result = self.value.advance_bytes(source, grant, false);
                if let Ok(CosStatus::Complete { end }) = result.status {
                    self.phase = MemberPhase::Tail { position: end, comment: false };
                    return CosReceipt { status: Ok(CosStatus::More), ..result };
                }
                if let Err(reason) = result.status { self.phase = MemberPhase::Failed(reason); }
                return result;
            }
            MemberPhase::Tail { mut position, mut comment } => {
                while receipt.work < grant.work {
                    if position == self.end {
                        self.phase = MemberPhase::Complete;
                        receipt.status = Ok(CosStatus::Complete { end: self.end });
                        return receipt;
                    }
                    let Some(byte) = source(position) else {
                        self.phase = MemberPhase::Failed("object stream member lost a decoded byte");
                        receipt.status = Err("object stream member lost a decoded byte");
                        return receipt;
                    };
                    position += 1;
                    receipt.work += 1;
                    if comment { comment = !matches!(byte, b'\r' | b'\n'); }
                    else if byte == b'%' { comment = true; }
                    else if !is_ws(byte) {
                        self.phase = MemberPhase::Failed("unexpected bytes after object stream member");
                        receipt.status = Err("unexpected bytes after object stream member");
                        return receipt;
                    }
                }
                self.phase = MemberPhase::Tail { position, comment };
                Ok(CosStatus::More)
            }
            _ => unreachable!("object stream member phase is active"),
        };
        receipt
    }

    /// 📤 Transfers the completed member's typed value.
    pub fn take_value(&mut self) -> Option<PdfObject> {
        if matches!(self.phase, MemberPhase::Complete) { self.value.take_value() } else { None }
    }

    /// ♻️ Begins retirement of the remaining typed owner.
    pub fn begin_close(&mut self) { self.phase = MemberPhase::Closing; self.value.begin_close(); }

    /// 🧹 Retires or reconciles the moved-out member ownership.
    pub fn close_step(&mut self, grant: CosGrant) -> CosReceipt { self.value.close_step(grant) }
}

/// 🚦 Selects, decodes and transfers one authored object stream member.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ObjectStreamEvent {
    Idle,
    More,
    NeedsAllocation { bytes: usize },
    NeedsRelease { bytes: usize },
    Complete { end: usize },
    Cancelled,
    Closed,
}

/// 🧮 Reports work and owned capacity changes for one borrowed-source turn.
#[derive(Debug, PartialEq, Eq)]
pub struct ObjectStreamReceipt {
    pub event: Result<ObjectStreamEvent, &'static str>,
    pub work: usize,
    pub allocated_bytes: usize,
    pub released_bytes: usize,
}

enum StreamPhase {
    Idle, Seek { number: u32, index: usize }, Boundary(usize), Decode, Retire,
    Failed(&'static str), Cancelled, Closing, Closed,
}

/// 🪟 Validates forward selections with one header pair of lookahead and one active COS owner.
#[must_use]
pub struct ObjectStreamCursor {
    phase: StreamPhase,
    header: ObjectStreamHeaderCursor,
    count: usize,
    next: usize,
    pair: Option<(u32, usize)>,
    member: Option<ObjectStreamMemberCursor>,
    limits: CosLimits,
}

impl ObjectStreamCursor {
    /// 🗂️ Prepares to select live members using the decoded stream's /N and /First.
    pub fn new(count: usize, first: usize, limits: CosLimits) -> Self {
        Self {
            phase: StreamPhase::Idle, header: ObjectStreamHeaderCursor::new(count, first),
            count, next: 0, pair: None, member: None, limits,
        }
    }

    /// 🎯 Requests an object number at the cross-reference entry's zero-based member index.
    pub fn select(&mut self, number: u32, index: u32) -> Result<(), &'static str> {
        if !matches!(self.phase, StreamPhase::Idle) { return Err("object stream selection is active or closed"); }
        let index = usize::try_from(index).map_err(|_| "object stream index exceeds its declared count")?;
        if index >= self.count { return Err("object stream index exceeds its declared count"); }
        if index < self.next { return Err("object stream selection must move forward"); }
        self.phase = StreamPhase::Seek { number, index };
        Ok(())
    }

    /// 🛤️ Drives one selection or its retirement while the caller keeps decoded bytes stable.
    pub fn advance(&mut self, decoded: &(impl ByteReadSource + ?Sized), grant: CosGrant, cancelled: bool) -> ObjectStreamReceipt {
        use ObjectStreamEvent as Event;
        if cancelled && !matches!(self.phase, StreamPhase::Closing | StreamPhase::Closed) { self.phase = StreamPhase::Cancelled; }
        match self.phase {
            StreamPhase::Idle => stream_step(Ok(Event::Idle), 0),
            StreamPhase::Cancelled => stream_step(Ok(Event::Cancelled), 0),
            StreamPhase::Closed => stream_step(Ok(Event::Closed), 0),
            StreamPhase::Failed(reason) => stream_step(Err(reason), 0),
            StreamPhase::Closing => stream_step(Ok(Event::More), 0),
            StreamPhase::Decode => stream_member_step(self.member.as_mut().expect("active object stream member").advance(decoded, grant, false)),
            StreamPhase::Retire => self.retire(grant),
            _ if grant.work == 0 => stream_step(Ok(Event::More), 0),
            StreamPhase::Seek { number, index } => {
                let Some((authored, start)) = self.pair else { return self.read_pair(decoded, grant.work); };
                if self.next == index && authored != number {
                    self.phase = StreamPhase::Idle;
                    return stream_step(Err("object stream member number differs from cross-reference"), 1);
                }
                if self.next == index { self.phase = StreamPhase::Boundary(start); }
                self.pair = None;
                self.next += 1;
                stream_step(Ok(Event::More), 1)
            }
            StreamPhase::Boundary(start) => {
                let end = if self.next == self.count {
                    decoded.byte_len()
                } else {
                    let Some((_, end)) = self.pair else { return self.read_pair(decoded, grant.work); };
                    end
                };
                self.member = Some(ObjectStreamMemberCursor::new(start, end, self.limits));
                self.phase = StreamPhase::Decode;
                stream_step(Ok(Event::More), 1)
            }
        }
    }

    fn read_pair(&mut self, decoded: &(impl ByteReadSource + ?Sized), work: usize) -> ObjectStreamReceipt {
        use ObjectStreamHeaderEvent as Header;
        let step = self.header.advance(decoded, work, false);
        let event = match step.event {
            Ok(Header::Member { number, offset }) => { self.pair = Some((number, offset)); Ok(ObjectStreamEvent::More) }
            Ok(Header::More) => Ok(ObjectStreamEvent::More),
            Err(reason) => Err(reason),
            _ => Err("object stream header ended before its declared count"),
        };
        if let Err(reason) = event { self.phase = StreamPhase::Failed(reason); }
        stream_step(event, step.work)
    }

    /// 📦 Moves the completed value into downstream custody and starts retiring parser scratch.
    pub fn take_value(&mut self) -> Option<PdfObject> {
        if !matches!(self.phase, StreamPhase::Decode) { return None; }
        let member = self.member.as_mut()?;
        let value = member.take_value()?;
        member.begin_close();
        self.phase = StreamPhase::Retire;
        Some(value)
    }

    /// 🧯 Starts retiring any unfinished or unclaimed member.
    pub fn begin_close(&mut self) {
        if matches!(self.phase, StreamPhase::Closed) { return; }
        if let Some(member) = self.member.as_mut() { member.begin_close(); }
        self.phase = StreamPhase::Closing;
    }

    /// 🧺 Releases retained parser capacity under the caller's grant.
    pub fn close_step(&mut self, grant: CosGrant) -> ObjectStreamReceipt {
        match self.phase {
            StreamPhase::Closing => self.retire(grant),
            StreamPhase::Closed => stream_step(Ok(ObjectStreamEvent::Closed), 0),
            _ => stream_step(Ok(ObjectStreamEvent::More), 0),
        }
    }

    fn retire(&mut self, grant: CosGrant) -> ObjectStreamReceipt {
        let mut step = self.member.as_mut().map_or_else(
            || stream_step(Ok(ObjectStreamEvent::Closed), 0),
            |member| stream_member_step(member.close_step(grant)),
        );
        if step.event == Ok(ObjectStreamEvent::Closed) {
            self.member = None;
            if matches!(self.phase, StreamPhase::Retire) {
                self.phase = StreamPhase::Idle;
                step.event = Ok(ObjectStreamEvent::Idle);
            } else {
                self.phase = StreamPhase::Closed;
            }
        }
        step
    }
}

fn stream_step(event: Result<ObjectStreamEvent, &'static str>, work: usize) -> ObjectStreamReceipt {
    ObjectStreamReceipt { event, work, allocated_bytes: 0, released_bytes: 0 }
}

fn stream_member_step(step: CosReceipt) -> ObjectStreamReceipt {
    use ObjectStreamEvent as Event;
    let event = step.status.map(|status| match status {
        CosStatus::More => Event::More,
        CosStatus::NeedsAllocation { bytes } => Event::NeedsAllocation { bytes },
        CosStatus::NeedsRelease { bytes } => Event::NeedsRelease { bytes },
        CosStatus::Complete { end } => Event::Complete { end },
        CosStatus::Cancelled => Event::Cancelled,
        CosStatus::Closed => Event::Closed,
    });
    ObjectStreamReceipt { event, work: step.work, allocated_bytes: step.allocated_bytes, released_bytes: step.released_bytes }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
