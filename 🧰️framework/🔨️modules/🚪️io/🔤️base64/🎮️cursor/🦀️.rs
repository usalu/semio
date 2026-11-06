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

/// 🧮️ The last caller turn reports actual payload visits, appended bytes and retained allocation.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Base64DecodeReceipt {
    pub work_units: usize,
    pub read_bytes: usize,
    pub copied_bytes: usize,
    pub allocated_capacity_bytes: usize,
}

/// ♻️ Consuming withdrawal returns every original owner, including an unfinished output prefix.
pub struct Base64DecodeParts<'input> {
    pub input: Base64Input<'input>,
    pub range: Base64InputRange,
    pub output: Option<Vec<u8>>,
    pub outcome: Base64DecodeOutcome,
}

/// 🖱️ Validation and reconstruction retain their exact positions across bounded caller turns.
pub struct Base64DecodeCursor<'input> {
    input: Base64Input<'input>,
    range: Base64InputRange,
    output: Option<Vec<u8>>,
    refusal: Option<Base64ControlError>,
    output_length: usize,
    phase: Base64Phase,
    offset: usize,
    scan_offset: usize,
    started: bool,
    complete: bool,
    last_turn: Base64DecodeReceipt,
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
        Self { input, range, output: None, refusal, output_length, phase: Base64Phase::Validate, offset: 0, scan_offset:0, started: false, complete: false, last_turn: Default::default() }
    }

    fn bytes(&self) -> &[u8] { if self.input.admits(self.range) { &self.input.bytes()[self.range.start..self.range.end] } else { &[] } }

    /// 📊️ Reports validated or reconstructed input octets in the current phase.
    pub fn progress(&self) -> Base64Progress { Base64Progress { phase: self.phase, completed: self.offset, total: self.bytes().len() } }

    /// 📏️ Charges byte visits, including the nonfinal padding scan, or one stage transition.
    pub fn next_work_demand(&self, maximum_work_units:usize) -> usize {
        if self.complete || self.refusal.is_some() { return 0; }
        let length = self.bytes().len();
        if !self.started || self.offset == length { return usize::from(maximum_work_units>0); }
        self.selected_input_bytes(maximum_work_units)
    }

    fn validation_block_end(&self)->usize{let length=self.bytes().len();(self.offset-self.offset%4096).saturating_add(4096).min(length)}
    fn padding_scan_pending(&self)->bool{self.phase==Base64Phase::Validate&&self.validation_block_end()<self.bytes().len()&&self.scan_offset<self.validation_block_end()}
    fn selected_input_bytes(&self,maximum_work_units:usize)->usize{
        if self.complete||self.refusal.is_some()||!self.started{return 0}
        let remaining=if self.padding_scan_pending(){self.validation_block_end().saturating_sub(self.scan_offset)}else if self.phase==Base64Phase::Validate{self.validation_block_end().saturating_sub(self.offset)}else{self.bytes().len().saturating_sub(self.offset)};
        4096usize.min(remaining).min((maximum_work_units/4)*4)
    }

    /// 🧾️ Quotes decoded bytes for `step(next_work_demand(maximum_work_units), ...)` under this finite grant.
    pub fn next_copy_byte_demand(&self,maximum_work_units:usize)->usize{
        if self.phase!=Base64Phase::Decode{return 0}
        let bytes=self.selected_input_bytes(maximum_work_units);
        (bytes/4*3).min(self.output_length.saturating_sub(self.output.as_ref().map_or(0,Vec::len)))
    }

    /// 🖨️ Borrows the exact decoded extent from length and final padding; strict validation still precedes reconstruction.
    pub fn expected_output_bytes(&self) -> usize { self.output_length }

    /// 🧾️ Borrows actual effects even when the current step returned its first refusal.
    pub fn last_turn_receipt(&self) -> Base64DecodeReceipt { self.last_turn }

    /// 🚫️ Borrows the original frozen refusal without advancing or fabricating work.
    pub fn refusal(&self)->Option<&Base64ControlError>{self.refusal.as_ref()}

    /// ⏭️ Quotes the one actual retained output allocation before its decode-stage birth.
    pub fn next_capacity_byte_demand(&self) -> usize {
        if !self.complete && self.refusal.is_none() && self.phase == Base64Phase::Decode && !self.started { self.output_length } else { 0 }
    }

    fn advance(&mut self, selected_bytes:usize, control: &mut Base64Control<'_>) -> Result<(), Base64ControlError> {
        if let Err(error)=control.admit(self.output_length){self.last_turn.work_units+=1;return Err(error);}
        let bytes = if self.input.admits(self.range) { &self.input.bytes()[self.range.start..self.range.end] } else { &[] };
        if !self.started {
            self.last_turn.work_units+=1;
            control.checkpoint(self.phase, 0, bytes.len())?;
            if self.phase == Base64Phase::Decode {
                let mut output = Vec::new();
                output.try_reserve_exact(self.output_length).map_err(|_| Base64ControlError::OutputLimit)?;
                self.last_turn.allocated_capacity_bytes+=output.capacity();
                self.output = Some(output);
            }
            self.started = true;
        } else if self.offset == bytes.len() {
            self.last_turn.work_units+=1;
            if self.phase == Base64Phase::Validate { self.phase = Base64Phase::Decode; self.offset = 0; self.scan_offset=0; self.started = false; }
            else { self.complete = true; }
        } else {
            let block_end=if self.phase==Base64Phase::Validate{(self.offset-self.offset%4096).saturating_add(4096).min(bytes.len())}else{0};
            if self.phase==Base64Phase::Validate&&block_end<bytes.len()&&self.scan_offset<block_end{
                let end=self.scan_offset+selected_bytes;let chunk=&bytes[self.scan_offset..end];
                for byte in chunk {
                    self.last_turn.work_units+=1;self.last_turn.read_bytes+=1;
                    if *byte==b'=' { return Err(Base64Error::InvalidPadding.into()); }
                }
                self.scan_offset=end;return Ok(());
            }
            let end = self.offset + selected_bytes;
            let chunk = &bytes[self.offset..end];
            for (index, quad) in chunk.as_chunks::<4>().0.iter().enumerate() {
                self.last_turn.work_units+=4;self.last_turn.read_bytes+=4;
                let index = self.offset + index * 4;
                let (value, count) = decode_standard_quad(*quad, index, index + 4 == bytes.len())?;
                if self.phase == Base64Phase::Decode { self.output.as_mut().unwrap().extend_from_slice(&value[..count]);self.last_turn.copied_bytes+=count; }
            }
            self.offset = end;
            if self.phase!=Base64Phase::Validate||end==block_end{control.checkpoint(self.phase, end, bytes.len())?;}
        }
        Ok(())
    }

    /// 🚦️ Zero or undersized grants are inert; the first refusal freezes all later work.
    pub fn step(&mut self, maximum_work_units: usize, control: &mut Base64Control<'_>) -> Result<Base64DecodeState, Base64ControlError> {
        self.last_turn=Default::default();
        if let Some(error) = &self.refusal { return Err(error.clone()); }
        let mut remaining = maximum_work_units;
        while !self.complete {
            let demand = self.next_work_demand(remaining);
            if demand == 0 { return Ok(Base64DecodeState::Pending); }
            let selected=self.selected_input_bytes(remaining);
            remaining -= demand;
            if let Err(error) = self.advance(selected,control) { self.refusal = Some(error.clone()); return Err(error); }
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

    /// 💎️ Borrows exact currently retained output capacity without transferring its owner.
    pub fn output_capacity(&self) -> usize { self.output.as_ref().map_or(0, Vec::capacity) }

    /// 📤️ Moves the original result exactly once, only after complete reconstruction.
    pub fn take_output(&mut self) -> Option<Vec<u8>> { if self.complete { self.output.take() } else { None } }

    /// 〽️ Transfers original custody to the caller's retirement mechanism without allocating or releasing.
    pub fn into_parts(self) -> Base64DecodeParts<'input> {
        let outcome = match self.refusal { Some(error) => Base64DecodeOutcome::Refused(error), None if self.complete => Base64DecodeOutcome::Complete, None => Base64DecodeOutcome::Pending };
        Base64DecodeParts { input: self.input, range: self.range, output: self.output, outcome }
    }
}
