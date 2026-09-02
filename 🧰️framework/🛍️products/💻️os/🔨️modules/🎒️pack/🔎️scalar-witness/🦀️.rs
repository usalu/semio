//! 🔎️ Allocation-free canonical operation-wire witness for fixed scalar records.

//#region 🔣️BorrowedRecord
/// 🧱️ Fixed inline scalar shape; Text borrows the caller's immutable typed owner.
#[derive(Clone, Copy)]
pub enum ScalarRecordField<'a> { Text(&'a str), U64(u64), F64(f64) }

/// 🪪️ Field ids are their array positions; absent fields are omitted by the canonical pack codec.
#[derive(Clone, Copy)]
pub struct ScalarRecordView<'a> { pub ordinal: u64, pub fields: [Option<ScalarRecordField<'a>>; 3] }

impl ScalarRecordView<'_> {
    fn text(&self, field: usize) -> Option<&str> { match self.fields.get(field).copied().flatten() { Some(ScalarRecordField::Text(value)) => Some(value), _ => None } }
}

/// 📊️ Consumed releases one input byte; Progress retains it while one source byte or metadata item advances.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScalarRecordWireStep { Progress { compared_bytes: usize }, Consumed { compared_bytes: usize }, Complete }
//#endregion 🔣️BorrowedRecord

//#region 🧵️Witness
#[derive(Clone, Copy, PartialEq, Eq)]
enum Phase { Init, Compare, Format, Ordinal, SymbolCount, SymbolLength, SymbolText, FieldCount, FieldId, FieldTag, FieldPayload, FieldText, Done, Fault, Closed }

/// 🧵️ No payload allocation, scan, or clone occurs during construction, advancement, or cancellation.
struct ScalarRecordWireCursor {
    phase: Phase, texts: [Option<usize>; 2], symbols: [Option<usize>; 2], string_symbols: [Option<u8>; 3],
    symbol_count: usize, symbol: usize, field: usize, offset: usize, varint: usize,
    compare_offset: usize, compare_left: Option<u8>, ordering: Option<std::cmp::Ordering>, pending: Option<u8>, consumed: usize,
}

impl Default for ScalarRecordWireCursor {
    fn default() -> Self {
        Self { phase: Phase::Init, texts: [None; 2], symbols: [None; 2], string_symbols: [None; 3], symbol_count: 0, symbol: 0, field: 0, offset: 0, varint: 0, compare_offset: 0, compare_left: None, ordering: None, pending: None, consumed: 0 }
    }
}

impl ScalarRecordWireCursor {
    fn symbols(&mut self, view: ScalarRecordView<'_>) {
        let Some(first) = self.texts[0] else { self.phase = Phase::Format; return; };
        if let Some(second) = self.texts[1] {
            if self.ordering == Some(std::cmp::Ordering::Equal) {
                self.symbols[0] = Some(first); self.symbol_count = 1; self.string_symbols[first] = Some(0); self.string_symbols[second] = Some(0);
            } else {
                let order = if self.ordering == Some(std::cmp::Ordering::Greater) { [second, first] } else { [first, second] };
                for field in order {
                    if view.text(field).unwrap().len() <= 128 { self.symbols[self.symbol_count] = Some(field); self.string_symbols[field] = Some(self.symbol_count as u8); self.symbol_count += 1; }
                }
            }
        } else if view.text(first).unwrap().len() <= 128 { self.symbols[0] = Some(first); self.symbol_count = 1; self.string_symbols[first] = Some(0); }
        self.phase = Phase::Format;
    }

    fn byte(&mut self, byte: u8) -> ScalarRecordWireStep { self.pending = Some(byte); ScalarRecordWireStep::Progress { compared_bytes: 1 } }

    fn varint_byte(&mut self, value: u64, next: Phase) -> ScalarRecordWireStep {
        let shift = self.varint * 7; let value = value >> shift; let mut byte = (value & 127) as u8;
        if value >= 128 { byte |= 128; self.varint += 1; } else { self.varint = 0; self.phase = next; }
        self.byte(byte)
    }

    /// 🧮️ Each call compares at most one source or input byte; the caller advances input only on Consumed.
    pub fn advance(&mut self, view: ScalarRecordView<'_>, input: Option<u8>) -> Result<ScalarRecordWireStep, &'static str> {
        use ScalarRecordWireStep as Step;
        if self.phase == Phase::Closed { return Err("scalar-record witness advanced after close"); }
        if let Some(expected) = self.pending.take() {
            if input != Some(expected) { return Err("scalar-record canonical wire differs from exact typed owner"); }
            self.consumed += 1; return Ok(Step::Consumed { compared_bytes: 1 });
        }
        match self.phase {
            Phase::Init => {
                let mut text_count = 0;
                for (field, value) in view.fields.iter().enumerate() {
                    if matches!(value, Some(ScalarRecordField::Text(_))) {
                        if text_count == 2 { return Err("scalar-record witness admits at most two text fields"); }
                        self.texts[text_count] = Some(field); text_count += 1;
                    }
                }
                if text_count == 2 { self.phase = Phase::Compare; } else { self.symbols(view); }
            }
            Phase::Compare => {
                let left = view.text(self.texts[0].unwrap()).ok_or("scalar-record first text owner changed")?;
                let right = view.text(self.texts[1].unwrap()).ok_or("scalar-record second text owner changed")?;
                if self.compare_offset == left.len().min(right.len()) {
                    self.ordering = Some(left.len().cmp(&right.len())); self.symbols(view);
                } else if let Some(left) = self.compare_left.take() {
                    let ordering = left.cmp(&right.as_bytes()[self.compare_offset]);
                    if ordering != std::cmp::Ordering::Equal { self.ordering = Some(ordering); self.symbols(view); } else { self.compare_offset += 1; }
                    return Ok(Step::Progress { compared_bytes: 1 });
                } else { self.compare_left = Some(left.as_bytes()[self.compare_offset]); return Ok(Step::Progress { compared_bytes: 1 }); }
            }
            Phase::Format => { self.phase = Phase::Ordinal; return Ok(self.byte(1)); }
            Phase::Ordinal => return Ok(self.varint_byte(view.ordinal, Phase::SymbolCount)),
            Phase::SymbolCount => return Ok(self.varint_byte(self.symbol_count as u64, if self.symbol_count == 0 { Phase::FieldCount } else { Phase::SymbolLength })),
            Phase::SymbolLength => {
                let field = self.symbols[self.symbol].ok_or("scalar-record symbol owner missing")?;
                return Ok(self.varint_byte(view.text(field).ok_or("scalar-record symbol type changed")?.len() as u64, Phase::SymbolText));
            }
            Phase::SymbolText => {
                let text = view.text(self.symbols[self.symbol].unwrap()).ok_or("scalar-record symbol type changed")?;
                if let Some(byte) = text.as_bytes().get(self.offset) { self.offset += 1; return Ok(self.byte(*byte)); }
                self.offset = 0; self.symbol += 1; self.phase = if self.symbol == self.symbol_count { Phase::FieldCount } else { Phase::SymbolLength };
            }
            Phase::FieldCount => return Ok(self.varint_byte(view.fields.iter().filter(|field| field.is_some()).count() as u64, Phase::FieldId)),
            Phase::FieldId => {
                if self.field == view.fields.len() { self.phase = Phase::Done; }
                else if view.fields[self.field].is_none() { self.field += 1; }
                else { return Ok(self.varint_byte(self.field as u64, Phase::FieldTag)); }
            }
            Phase::FieldTag => {
                self.phase = Phase::FieldPayload;
                let tag = match view.fields[self.field].ok_or("scalar-record field disappeared")? {
                    ScalarRecordField::Text(_) => if self.string_symbols[self.field].is_some() { 6 } else { 7 }, ScalarRecordField::U64(_) => 4, ScalarRecordField::F64(_) => 5,
                };
                return Ok(self.byte(tag));
            }
            Phase::FieldPayload => match view.fields[self.field].ok_or("scalar-record field disappeared")? {
                ScalarRecordField::Text(text) => {
                    if let Some(symbol) = self.string_symbols[self.field] {
                        let result = self.varint_byte(symbol as u64, Phase::FieldId); if self.phase == Phase::FieldId { self.field += 1; } return Ok(result);
                    }
                    return Ok(self.varint_byte(text.len() as u64, Phase::FieldText));
                }
                ScalarRecordField::U64(value) => { let result = self.varint_byte(value, Phase::FieldId); if self.phase == Phase::FieldId { self.field += 1; } return Ok(result); }
                ScalarRecordField::F64(value) => {
                    let bits = if value.is_nan() { 0x7ff8_0000_0000_0000u64 } else { value.to_bits() };
                    let byte = bits.to_le_bytes()[self.offset]; self.offset += 1;
                    if self.offset == 8 { self.offset = 0; self.field += 1; self.phase = Phase::FieldId; }
                    return Ok(self.byte(byte));
                }
            },
            Phase::FieldText => {
                let text = view.text(self.field).ok_or("scalar-record inline text type changed")?;
                if let Some(byte) = text.as_bytes().get(self.offset) { self.offset += 1; return Ok(self.byte(*byte)); }
                self.offset = 0; self.field += 1; self.phase = Phase::FieldId;
            }
            Phase::Done => { if input.is_some() { return Err("scalar-record wire has trailing bytes"); } return Ok(Step::Complete); }
            Phase::Fault => return Err("scalar-record witness remains faulted until close"),
            Phase::Closed => unreachable!(),
        }
        Ok(Step::Progress { compared_bytes: 0 })
    }

    pub fn consumed_bytes(&self) -> usize { self.consumed }
    pub fn begin_close(&mut self) { self.phase = Phase::Closed; self.pending = None; self.compare_left = None; self.texts = [None; 2]; self.symbols = [None; 2]; self.string_symbols = [None; 3]; }
    pub fn terminal_is_empty(&self) -> bool { self.phase == Phase::Closed && self.pending.is_none() && self.compare_left.is_none() }
}
//#endregion 🧵️Witness

//#region 🪪️FrozenOwner
/// 🪪️ Captures one fixed scalar projection from its exact retained root and transfers that root for domain retirement after close.
/// Private string borrows are valid while the frozen Arc remains retained, never escape, and are cleared before any root transfer.
/// Scalars are copied once, so even a source with atomic metadata cannot change an admitted wire projection between steps.
pub struct ScalarRecordWireWitness<R: Send + Sync> {
    root: std::mem::ManuallyDrop<Option<std::sync::Arc<R>>>,
    project: for<'a> fn(&'a R) -> Result<ScalarRecordView<'a>, &'static str>,
    view: Option<ScalarRecordView<'static>>,
    cursor: ScalarRecordWireCursor,
}
impl<R: Send + Sync> ScalarRecordWireWitness<R> {
    pub fn new(root: std::sync::Arc<R>, project: for<'a> fn(&'a R) -> Result<ScalarRecordView<'a>, &'static str>) -> Self {
        Self { root: std::mem::ManuallyDrop::new(Some(root)), project, view: None, cursor: ScalarRecordWireCursor::default() }
    }
    pub fn advance(&mut self, input: Option<u8>) -> Result<ScalarRecordWireStep, &'static str> {
        if matches!(self.cursor.phase, Phase::Fault | Phase::Closed) { return Err("scalar-record witness cannot resume after error or close"); }
        if self.view.is_none() {
            let projected = self.root.as_ref().ok_or("scalar-record exact root missing").and_then(|root| (self.project)(root));
            match projected {
                Ok(view) => self.view = Some(unsafe { std::mem::transmute::<ScalarRecordView<'_>, ScalarRecordView<'static>>(view) }),
                Err(error) => { self.cursor.phase = Phase::Fault; return Err(error); }
            }
            return Ok(ScalarRecordWireStep::Progress { compared_bytes: 0 });
        }
        let result = self.cursor.advance(self.view.unwrap(), input);
        if result.is_err() { self.cursor.pending = None; self.cursor.phase = Phase::Fault; }
        result
    }
    pub fn consumed_bytes(&self) -> usize { self.cursor.consumed_bytes() }
    pub fn begin_close(&mut self) { self.view = None; self.cursor.begin_close(); }
    pub fn take_root(&mut self) -> Option<std::sync::Arc<R>> { if self.view.is_none() && self.cursor.terminal_is_empty() { self.root.take() } else { None } }
    pub fn terminal_is_empty(&self) -> bool { self.root.is_none() && self.view.is_none() && self.cursor.terminal_is_empty() }
}
impl<R: Send + Sync> Drop for ScalarRecordWireWitness<R> {
    fn drop(&mut self) {
        if self.terminal_is_empty() { unsafe { std::mem::ManuallyDrop::drop(&mut self.root); } }
        else if !std::thread::panicking() { panic!("scalar-record witness must close and transfer its exact root before drop"); }
    }
}
//#endregion 🪪️FrozenOwner

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
