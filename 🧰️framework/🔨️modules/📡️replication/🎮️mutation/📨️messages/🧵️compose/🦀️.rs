//! 🧵️ Composes bounded diagnostic text directly from immutable native UTF8 capabilities.
use super::{MutationMessage, MUTATION_MESSAGE_ENTRY_BYTES, MUTATION_MESSAGE_OWNER_BYTES, MUTATION_MESSAGE_TARGET_BYTES};
use semio_framework_diagnostic::Severity;
use semio_framework_value::{paged::Utf8Text, ValueError, retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}};
use std::mem::{ManuallyDrop, size_of};

/// 🧩️ Text projections and scalar formatting never require a contiguous original owner.
pub enum ArtifactMessageFragment<'a> { Static(&'static str), Text(&'a dyn Utf8Text), JsonQuoted(&'a dyn Utf8Text), Unsigned(u64) }

/// 🪪️ One immutable source stays at the same address and exposes constant-work metadata.
pub trait ArtifactMessageSource {
    fn level(&self) -> Severity;
    fn code(&self) -> &'static str;
    fn operation_index(&self) -> Option<u32>;
    fn fragment_count(&self) -> usize;
    fn fragment(&self, index: usize) -> Option<ArtifactMessageFragment<'_>>;
    fn target_count(&self) -> usize;
    fn target(&self, index: usize) -> Option<&dyn Utf8Text>;
}

/// 🚫️ Refusals are allocation-free; the caller still owns and closes the partial cursor.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArtifactMessageComposeRefusal { SourceChanged, CodeTooLong, InvalidBudget, InvalidSource, AllocationFailed, CapacityExceeded }

#[derive(Clone, Copy, PartialEq, Eq)]
enum Phase { Bind, Scaffold, CodeReserve, Code, Target, TargetReserve, TargetCopy, BodyReserve, Body, Complete }

#[derive(Default)]
struct Span { chunk: usize, offset: usize, quoted: u8, escaped: usize }

enum Token { Scalar(char), Boundary, End }

#[derive(Clone, Copy, PartialEq, Eq)]
struct SourceStamp { level: Severity, code: &'static str, operation: Option<u32>, fragments: usize, targets: usize }

#[derive(Clone, Copy, PartialEq, Eq)]
enum FragmentStamp { Static(usize, usize), Text(usize, usize, usize, bool), Unsigned(u64) }

impl FragmentStamp {
    fn read(fragment: &ArtifactMessageFragment<'_>) -> Self {
        match fragment {
            ArtifactMessageFragment::Static(text) => Self::Static(text.as_ptr() as usize, text.len()),
            ArtifactMessageFragment::Text(text) | ArtifactMessageFragment::JsonQuoted(text) => Self::Text(std::ptr::from_ref(*text).cast::<()>() as usize, text.text_bytes(), text.text_chunk_count(), matches!(fragment, ArtifactMessageFragment::JsonQuoted(_))),
            ArtifactMessageFragment::Unsigned(value) => Self::Unsigned(*value),
        }
    }
}

impl SourceStamp {
    fn read(source: &impl ArtifactMessageSource) -> Self { Self { level: source.level(), code: source.code(), operation: source.operation_index(), fragments: source.fragment_count(), targets: source.target_count() } }
}

fn json_escape(scalar: char) -> Option<([u8; 6], usize)> {
    let escape = match scalar { '"' => Some(b'"'), '\\' => Some(b'\\'), '\n' => Some(b'n'), '\r' => Some(b'r'), '\t' => Some(b't'), '\u{8}' => Some(b'b'), '\u{c}' => Some(b'f'), _ => None };
    if let Some(escape) = escape { return Some(([b'\\', escape, 0, 0, 0, 0], 2)); }
    (scalar <= '\u{1f}').then(|| ([b'\\', b'u', b'0', b'0', b"0123456789abcdef"[(scalar as usize) >> 4], b"0123456789abcdef"[(scalar as usize) & 15]], 6))
}

impl Span {
    fn read(&self, text: &dyn Utf8Text) -> Result<Token, ArtifactMessageComposeRefusal> {
        if self.chunk >= text.text_chunk_count() { return Ok(Token::End); }
        let chunk = text.text_chunk(self.chunk).ok_or(ArtifactMessageComposeRefusal::InvalidSource)?;
        let tail = chunk.get(self.offset..).ok_or(ArtifactMessageComposeRefusal::InvalidSource)?;
        Ok(tail.chars().next().map_or(Token::Boundary, Token::Scalar))
    }
    fn consume(&mut self, scalar: char) { self.offset += scalar.len_utf8(); }
    fn next_chunk(&mut self) { self.chunk += 1; self.offset = 0; }
}

/// 📨️ Only visible bounded owners are allocated, under separate copy and capacity grants.
pub struct ArtifactMessageComposeCursor {
    output: ManuallyDrop<Option<MutationMessage>>,
    source: Option<usize>,
    stamp: Option<SourceStamp>,
    phase: Phase,
    limit: usize,
    remaining: usize,
    slots: usize,
    target: usize,
    target_bytes: usize,
    target_source: Option<usize>,
    fragment: usize,
    fragment_source: Option<FragmentStamp>,
    span: Span,
    cancelled: bool,
    closing: bool,
    truncated: bool,
}

impl ArtifactMessageComposeCursor {
    pub fn new(maximum_entry_bytes: usize) -> Self {
        Self { output: ManuallyDrop::new(None), source: None, stamp: None, phase: Phase::Bind, limit: maximum_entry_bytes, remaining: 0, slots: 0, target: 0, target_bytes: 0, target_source: None, fragment: 0, fragment_source: None, span: Span::default(), cancelled: false, closing: false, truncated: false }
    }

    fn check(&self, source: &impl ArtifactMessageSource) -> Result<(), ArtifactMessageComposeRefusal> {
        if self.source.is_some_and(|address| address != std::ptr::from_ref(source) as usize) { return Err(ArtifactMessageComposeRefusal::SourceChanged); }
        if let Some(stamp) = self.stamp {
            let current = SourceStamp::read(source);
            if current.code.len() > 64 || current != stamp { return Err(ArtifactMessageComposeRefusal::SourceChanged); }
        }
        Ok(())
    }

    fn target_source<'a>(&self, source: &'a impl ArtifactMessageSource) -> Result<&'a dyn Utf8Text, ArtifactMessageComposeRefusal> {
        let target = source.target(self.target).ok_or(ArtifactMessageComposeRefusal::InvalidSource)?;
        if self.target_source != Some(std::ptr::from_ref(target).cast::<()>() as usize) || self.target_bytes != target.text_bytes() { return Err(ArtifactMessageComposeRefusal::SourceChanged); }
        Ok(target)
    }

    pub fn next_capacity_byte_demand(&self, source: &impl ArtifactMessageSource) -> Result<usize, ArtifactMessageComposeRefusal> {
        self.check(source)?;
        Ok(match self.phase {
            Phase::Scaffold => self.slots * size_of::<String>(),
            Phase::CodeReserve => source.code().len(),
            Phase::TargetReserve => self.target_source(source)?.text_bytes(),
            Phase::BodyReserve => self.remaining,
            _ => 0,
        })
    }

    pub fn next_copy_byte_demand(&self, source: &impl ArtifactMessageSource) -> Result<usize, ArtifactMessageComposeRefusal> {
        self.check(source)?;
        Ok(match self.phase {
            Phase::Code => source.code().get(self.output.as_ref().map_or(0, |output| output.code.0.len())..).ok_or(ArtifactMessageComposeRefusal::InvalidSource)?.chars().next().map_or(0, char::len_utf8),
            Phase::TargetCopy => match self.span.read(self.target_source(source)?)? { Token::Scalar(scalar) => scalar.len_utf8(), _ => 0 },
            Phase::Body => self.body_token(source)?.map_or(0, |(_, length)| length),
            _ => 0,
        })
    }

    fn body_token(&self, source: &impl ArtifactMessageSource) -> Result<Option<([u8; 6], usize)>, ArtifactMessageComposeRefusal> {
        let Some(fragment) = source.fragment(self.fragment) else {
            return if self.fragment == source.fragment_count() { Ok(None) } else { Err(ArtifactMessageComposeRefusal::InvalidSource) };
        };
        if self.fragment_source.is_some_and(|expected| expected != FragmentStamp::read(&fragment)) { return Err(ArtifactMessageComposeRefusal::SourceChanged); }
        let mut bytes = [0; 6];
        let scalar = match fragment {
            ArtifactMessageFragment::Static(text) => text.get(self.span.offset..).ok_or(ArtifactMessageComposeRefusal::InvalidSource)?.chars().next(),
            ArtifactMessageFragment::Text(text) => match self.span.read(text)? { Token::Scalar(scalar) => Some(scalar), _ => None },
            ArtifactMessageFragment::JsonQuoted(text) => {
                if self.span.quoted == 0 || self.span.quoted == 2 { bytes[0] = b'"'; return Ok(Some((bytes, 1))); }
                match self.span.read(text)? {
                    Token::Scalar(scalar) => {
                        if let Some((escaped, length)) = json_escape(scalar) {
                            if self.span.escaped >= length { return Err(ArtifactMessageComposeRefusal::InvalidSource); }
                            bytes[0] = escaped[self.span.escaped]; return Ok(Some((bytes, 1)));
                        }
                        Some(scalar)
                    }
                    _ => None,
                }
            }
            ArtifactMessageFragment::Unsigned(value) => {
                let digits = if value == 0 { 1 } else { value.ilog10() as usize + 1 };
                if self.span.offset == digits { return Ok(None); }
                bytes[0] = b'0' + ((value / 10u64.pow((digits - self.span.offset - 1) as u32)) % 10) as u8;
                return Ok(Some((bytes, 1)));
            }
        };
        Ok(scalar.map(|scalar| { let length = scalar.encode_utf8(&mut bytes).len(); (bytes, length) }))
    }

    fn advance_body_position(&mut self, source: &impl ArtifactMessageSource) -> Result<(), ArtifactMessageComposeRefusal> {
        match source.fragment(self.fragment).ok_or(ArtifactMessageComposeRefusal::InvalidSource)? {
            ArtifactMessageFragment::Static(text) => self.span.consume(text[self.span.offset..].chars().next().ok_or(ArtifactMessageComposeRefusal::InvalidSource)?),
            ArtifactMessageFragment::Text(text) => if let Token::Scalar(scalar) = self.span.read(text)? { self.span.consume(scalar); } else { return Err(ArtifactMessageComposeRefusal::InvalidSource); },
            ArtifactMessageFragment::JsonQuoted(text) => match self.span.quoted {
                0 => self.span.quoted = 1,
                2 => self.span.quoted = 3,
                _ => if let Token::Scalar(scalar) = self.span.read(text)? {
                    if let Some((_, length)) = json_escape(scalar) {
                        self.span.escaped += 1;
                        if self.span.escaped == length { self.span.escaped = 0; self.span.consume(scalar); }
                    } else { self.span.consume(scalar); }
                } else { return Err(ArtifactMessageComposeRefusal::InvalidSource); },
            },
            ArtifactMessageFragment::Unsigned(_) => self.span.offset += 1,
        }
        Ok(())
    }

    fn advance_body_boundary(&mut self, source: &impl ArtifactMessageSource) -> Result<(), ArtifactMessageComposeRefusal> {
        let fragment = source.fragment(self.fragment).ok_or(ArtifactMessageComposeRefusal::InvalidSource)?;
        let complete = match fragment {
            ArtifactMessageFragment::Text(text) => match self.span.read(text)? { Token::Boundary => { self.span.next_chunk(); false }, Token::End => true, _ => return Err(ArtifactMessageComposeRefusal::InvalidSource) },
            ArtifactMessageFragment::JsonQuoted(text) => {
                if self.span.quoted == 3 { true }
                else { match self.span.read(text)? { Token::Boundary => self.span.next_chunk(), Token::End => self.span.quoted = 2, _ => return Err(ArtifactMessageComposeRefusal::InvalidSource) }; false }
            }
            _ => true,
        };
        if complete { self.fragment += 1; self.fragment_source = None; self.span = Span::default(); }
        Ok(())
    }

    pub fn advance(&mut self, source: &impl ArtifactMessageSource, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ArtifactMessageComposeRefusal> {
        self.check(source)?;
        if self.cancelled || self.closing || (grant.maximum_items == 0 || grant.maximum_depth == 0) { return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default())); }
        if self.phase == Phase::Complete { return Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default())); }
        let mut progress = RetainedCloneProgress { copied_items: 1, ..Default::default() };
        let demand = self.next_capacity_byte_demand(source)?;
        if demand > grant.maximum_capacity_bytes { return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default())); }
        match self.phase {
            Phase::Bind => {
                if source.code().len() > 64 { return Err(ArtifactMessageComposeRefusal::CodeTooLong); }
                if self.limit > MUTATION_MESSAGE_ENTRY_BYTES || self.limit < MUTATION_MESSAGE_OWNER_BYTES + source.code().len() { return Err(ArtifactMessageComposeRefusal::InvalidBudget); }
                self.source = Some(std::ptr::from_ref(source) as usize);
                self.stamp = Some(SourceStamp::read(source));
                self.remaining = self.limit - MUTATION_MESSAGE_OWNER_BYTES - source.code().len();
                self.slots = source.target_count().min(self.remaining / MUTATION_MESSAGE_TARGET_BYTES);
                *self.output = Some(MutationMessage { level: source.level(), code: String::new().into(), message: String::new(), target: Vec::new(), op_index: source.operation_index() });
                self.phase = Phase::Scaffold;
            }
            Phase::Scaffold => {
                let output = self.output.as_mut().ok_or(ArtifactMessageComposeRefusal::InvalidSource)?;
                output.target.try_reserve_exact(self.slots).map_err(|_| ArtifactMessageComposeRefusal::AllocationFailed)?;
                progress.retained_capacity_bytes = output.target.capacity() * size_of::<String>();
                self.phase = Phase::CodeReserve;
            }
            Phase::CodeReserve | Phase::TargetReserve | Phase::BodyReserve => {
                let output = self.output.as_mut().ok_or(ArtifactMessageComposeRefusal::InvalidSource)?;
                let destination = match self.phase { Phase::CodeReserve => &mut output.code.0, Phase::TargetReserve => output.target.last_mut().ok_or(ArtifactMessageComposeRefusal::InvalidSource)?, _ => &mut output.message };
                destination.try_reserve_exact(demand).map_err(|_| ArtifactMessageComposeRefusal::AllocationFailed)?;
                progress.retained_capacity_bytes = destination.capacity();
                self.phase = match self.phase { Phase::CodeReserve => Phase::Code, Phase::TargetReserve => Phase::TargetCopy, _ => Phase::Body };
            }
            Phase::Code => {
                let output = self.output.as_mut().ok_or(ArtifactMessageComposeRefusal::InvalidSource)?;
                if let Some(scalar) = source.code().get(output.code.0.len()..).ok_or(ArtifactMessageComposeRefusal::InvalidSource)?.chars().next() {
                    if scalar.len_utf8() > grant.maximum_copy_bytes { return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default())); }
                    output.code.0.push(scalar); progress.copied_bytes = scalar.len_utf8();
                } else { self.phase = Phase::Target; }
            }
            Phase::Target => {
                if self.target == source.target_count() { self.span = Span::default(); self.phase = Phase::BodyReserve; }
                else {
                    let target = source.target(self.target).ok_or(ArtifactMessageComposeRefusal::InvalidSource)?;
                    let demand = target.text_bytes().checked_add(MUTATION_MESSAGE_TARGET_BYTES);
                    if demand.is_some_and(|demand| demand <= self.remaining) {
                        self.target_bytes = target.text_bytes();
                        self.target_source = Some(std::ptr::from_ref(target).cast::<()>() as usize);
                        self.remaining -= demand.unwrap();
                        let output = self.output.as_mut().ok_or(ArtifactMessageComposeRefusal::InvalidSource)?;
                        if output.target.len() == self.slots { return Err(ArtifactMessageComposeRefusal::InvalidSource); }
                        output.target.push(String::new()); self.span = Span::default(); self.phase = Phase::TargetReserve;
                    } else { self.target += 1; }
                }
            }
            Phase::TargetCopy => {
                let target = self.target_source(source)?;
                match self.span.read(target)? {
                    Token::Scalar(scalar) => {
                        if scalar.len_utf8() > grant.maximum_copy_bytes { return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default())); }
                        let destination = self.output.as_mut().and_then(|output| output.target.last_mut()).ok_or(ArtifactMessageComposeRefusal::InvalidSource)?;
                        if destination.len().saturating_add(scalar.len_utf8()) > target.text_bytes() { return Err(ArtifactMessageComposeRefusal::InvalidSource); }
                        destination.push(scalar); self.span.consume(scalar); progress.copied_bytes = scalar.len_utf8();
                    }
                    Token::Boundary => self.span.next_chunk(),
                    Token::End => {
                        if self.output.as_ref().and_then(|output| output.target.last()).map(String::len) != Some(target.text_bytes()) { return Err(ArtifactMessageComposeRefusal::InvalidSource); }
                        self.target += 1; self.target_source = None; self.target_bytes = 0; self.phase = Phase::Target;
                    }
                }
            }
            Phase::Body => {
                if self.fragment == source.fragment_count() { self.phase = Phase::Complete; }
                else if self.fragment_source.is_none() {
                    self.fragment_source = Some(FragmentStamp::read(&source.fragment(self.fragment).ok_or(ArtifactMessageComposeRefusal::InvalidSource)?));
                }
                else if let Some((bytes, length)) = self.body_token(source)? {
                    if length > self.remaining { self.truncated = true; self.phase = Phase::Complete; }
                    else {
                        if length > grant.maximum_copy_bytes { return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default())); }
                        let text = std::str::from_utf8(&bytes[..length]).map_err(|_| ArtifactMessageComposeRefusal::InvalidSource)?;
                        self.output.as_mut().ok_or(ArtifactMessageComposeRefusal::InvalidSource)?.message.push_str(text);
                        self.remaining -= length; self.advance_body_position(source)?; progress.copied_bytes = length;
                    }
                } else { self.advance_body_boundary(source)?; }
            }
            Phase::Complete => {}
        }
        if progress.retained_capacity_bytes > grant.maximum_capacity_bytes { return Err(ArtifactMessageComposeRefusal::CapacityExceeded); }
        Ok(if self.phase == Phase::Complete { RetainedCloneStep::Complete(progress) } else { RetainedCloneStep::Progress(progress) })
    }

    pub fn truncated(&self) -> bool { self.truncated }
    pub fn take(&mut self) -> Option<MutationMessage> { if self.phase == Phase::Complete && !self.cancelled && !self.closing { self.output.take() } else { None } }
    pub fn cancel(&mut self) { self.cancelled = true; }
    pub fn begin_close(&mut self) { self.closing = true; }

    pub fn next_release_byte_demand(&self) -> Result<usize, ValueError> {
        let Some(output) = self.output.as_ref() else { return Ok(0) };
        Ok(if let Some(target) = output.target.last() { target.capacity() }
        else if output.target.capacity() != 0 { output.target.capacity() * size_of::<String>() }
        else if output.code.0.capacity() != 0 { output.code.0.capacity() }
        else { output.message.capacity() })
    }

    pub fn next_depth_demand(&self) -> usize { usize::from(!self.terminal_is_empty()) }

    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default())); }
        if !self.closing || grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default())); }
        let demand = self.next_release_byte_demand()?;
        if demand > grant.maximum_release_bytes { return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default())); }
        if let Some(output) = self.output.as_mut() {
            if !output.target.is_empty() { drop(output.target.pop()); }
            else if output.target.capacity() != 0 { drop(std::mem::take(&mut output.target)); }
            else if output.code.0.capacity() != 0 { drop(std::mem::take(&mut output.code.0)); }
            else if output.message.capacity() != 0 { drop(std::mem::take(&mut output.message)); }
            else { self.output.take(); }
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: demand, ..Default::default() }));
        }
        self.source = None;
        self.stamp = None;
        self.target_source = None;
        self.fragment_source = None;
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..Default::default() }))
    }

    pub fn terminal_is_empty(&self) -> bool { self.closing && self.source.is_none() && self.output.is_none() }
}

impl semio_framework_value::ErasedSnapshotRetirement for ArtifactMessageComposeCursor {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> { Self::close_step(self, grant) }
    fn terminal_is_empty(&self) -> bool { Self::terminal_is_empty(self) }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(0) }
    fn next_capacity_byte_demand(&self, _: usize) -> Result<usize, ValueError> { Ok(0) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Self::next_release_byte_demand(self) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(Self::next_depth_demand(self)) }
}

impl Drop for ArtifactMessageComposeCursor {
    fn drop(&mut self) { let empty = self.terminal_is_empty(); assert!(empty || std::thread::panicking(), "borrowed diagnostic cursor dropped before exact ownership closure"); if empty { unsafe { ManuallyDrop::drop(&mut self.output); } } }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
