//! 🪜️ Retained strict decoding transfers exact input and partial output ownership between work grants.
use super::{Base64Control, Base64ControlError, Base64Error, Base64Phase, Base64Progress, decode_standard_quad};

/// 🫴️ Immutable source custody transfers into decoding without a payload copy.
pub enum Base64Input<'input> {
    Borrowed(&'input [u8]),
    OwnedBytes(Vec<u8>),
    OwnedText(String),
}

impl Base64Input<'_> {
    fn bytes(&self) -> &[u8] {
        match self { Self::Borrowed(bytes) => bytes, Self::OwnedBytes(bytes) => bytes, Self::OwnedText(text) => text.as_bytes() }
    }
    fn admits(&self, range: Base64InputRange) -> bool {
        range.start <= range.end && range.end <= self.bytes().len() && match self {
            Self::OwnedText(text) => text.is_char_boundary(range.start) && text.is_char_boundary(range.end),
            _ => true,
        }
    }
}

/// 📐️ Half-open byte addresses select a payload while retaining the complete source owner.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Base64InputRange { pub start: usize, pub end: usize }

/// 📍️ Completion permits one output transfer; pending requires another admitted work grant.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Base64DecodeState { Pending, Complete }

/// 🚪️ One tagged outcome distinguishes withdrawal, completion and the exact first refusal.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Base64DecodeOutcome { Pending, Complete, Refused(Base64ControlError) }

/// ♻️ Consuming withdrawal returns every original owner, including an unfinished output prefix.
pub struct Base64DecodeParts<'input> {
    pub input: Base64Input<'input>,
    pub range: Base64InputRange,
    pub output: Option<Vec<u8>>,
    pub outcome: Base64DecodeOutcome,
}

/// 🪜️ Validation and reconstruction retain their exact positions across bounded caller turns.
pub struct Base64DecodeCursor<'input> {
    input: Base64Input<'input>,
    range: Base64InputRange,
    output: Option<Vec<u8>>,
    refusal: Option<Base64ControlError>,
    output_length: usize,
    phase: Base64Phase,
    offset: usize,
    started: bool,
    complete: bool,
}

impl<'input> Base64DecodeCursor<'input> {
    /// 🎟️ Takes original custody using only length and the final two padding octets.
    pub fn new(input: Base64Input<'input>) -> Self {
        let range = Base64InputRange { start: 0, end: input.bytes().len() };
        Self::new_range(input, range)
    }

    /// 🎯️ Checks source bounds and text boundaries without copying or removing wrapper bytes.
    pub fn new_range(input: Base64Input<'input>, range: Base64InputRange) -> Self {
        let admitted = input.admits(range);
        let bytes = if admitted { &input.bytes()[range.start..range.end] } else { &[] };
        let valid_length = bytes.len().is_multiple_of(4);
        let padding = if bytes.ends_with(b"==") { 2 } else if bytes.ends_with(b"=") { 1 } else { 0 };
        let output_length = if valid_length { bytes.len() / 4 * 3 - padding } else { 0 };
        let refusal = if !admitted { Some(Base64ControlError::InputRange) } else { (!valid_length).then_some(Base64ControlError::Codec(Base64Error::InvalidLength)) };
        Self { input, range, output: None, refusal, output_length, phase: Base64Phase::Validate, offset: 0, started: false, complete: false }
    }

    fn bytes(&self) -> &[u8] { if self.input.admits(self.range) { &self.input.bytes()[self.range.start..self.range.end] } else { &[] } }

    /// 📊️ Reports validated or reconstructed input octets in the current phase.
    pub fn progress(&self) -> Base64Progress { Base64Progress { phase: self.phase, completed: self.offset, total: self.bytes().len() } }

    /// 📏️ Charges byte visits, including the nonfinal padding scan, or one stage transition.
    pub fn next_work_demand(&self) -> usize {
        if self.complete || self.refusal.is_some() { return 0; }
        let length = self.bytes().len();
        if !self.started || self.offset == length { return 1; }
        let bytes = (length - self.offset).min(4096);
        bytes + if self.phase == Base64Phase::Validate && self.offset + bytes < length { bytes } else { 0 }
    }

    fn advance(&mut self, control: &mut Base64Control<'_>) -> Result<(), Base64ControlError> {
        control.admit(self.output_length)?;
        let bytes = if self.input.admits(self.range) { &self.input.bytes()[self.range.start..self.range.end] } else { &[] };
        if !self.started {
            control.checkpoint(self.phase, 0, bytes.len())?;
            if self.phase == Base64Phase::Decode {
                let mut output = Vec::new();
                output.try_reserve_exact(self.output_length).map_err(|_| Base64ControlError::OutputLimit)?;
                self.output = Some(output);
            }
            self.started = true;
        } else if self.offset == bytes.len() {
            if self.phase == Base64Phase::Validate { self.phase = Base64Phase::Decode; self.offset = 0; self.started = false; }
            else { self.complete = true; }
        } else {
            let end = (self.offset + 4096).min(bytes.len());
            let chunk = &bytes[self.offset..end];
            if self.phase == Base64Phase::Validate && end < bytes.len() && chunk.contains(&b'=') { return Err(Base64Error::InvalidPadding.into()); }
            for (index, quad) in chunk.as_chunks::<4>().0.iter().enumerate() {
                let index = self.offset + index * 4;
                let (value, count) = decode_standard_quad(*quad, index, index + 4 == bytes.len())?;
                if self.phase == Base64Phase::Decode { self.output.as_mut().unwrap().extend_from_slice(&value[..count]); }
            }
            self.offset = end;
            control.checkpoint(self.phase, end, bytes.len())?;
        }
        Ok(())
    }

    /// 🚦️ Zero or undersized grants are inert; the first refusal freezes all later work.
    pub fn step(&mut self, maximum_work_units: usize, control: &mut Base64Control<'_>) -> Result<Base64DecodeState, Base64ControlError> {
        if let Some(error) = &self.refusal { return Err(error.clone()); }
        let mut remaining = maximum_work_units;
        while !self.complete {
            let demand = self.next_work_demand();
            if remaining < demand { return Ok(Base64DecodeState::Pending); }
            remaining -= demand;
            if let Err(error) = self.advance(control) { self.refusal = Some(error.clone()); return Err(error); }
        }
        Ok(Base64DecodeState::Complete)
    }

    /// 🛑️ Records cancellation without destroying any retained source or output allocation.
    pub fn cancel(&mut self) -> bool {
        if self.complete || self.refusal.is_some() { return false; }
        self.refusal = Some(Base64ControlError::Cancelled);
        true
    }

    /// ✅️ Indicates that both strict passes have completed successfully.
    pub fn is_complete(&self) -> bool { self.complete }

    /// 🔎️ Borrows the same immutable source owner for independent inspection.
    pub fn input(&self) -> &Base64Input<'input> { &self.input }

    /// 📏️ Borrows exact currently retained output capacity without transferring its owner.
    pub fn output_capacity(&self) -> usize { self.output.as_ref().map_or(0, Vec::capacity) }

    /// 📤️ Moves the original result exactly once, only after complete reconstruction.
    pub fn take_output(&mut self) -> Option<Vec<u8>> { if self.complete { self.output.take() } else { None } }

    /// 🫴️ Transfers original custody to the caller's retirement mechanism without allocating or releasing.
    pub fn into_parts(self) -> Base64DecodeParts<'input> {
        let outcome = match self.refusal { Some(error) => Base64DecodeOutcome::Refused(error), None if self.complete => Base64DecodeOutcome::Complete, None => Base64DecodeOutcome::Pending };
        Base64DecodeParts { input: self.input, range: self.range, output: self.output, outcome }
    }
}
