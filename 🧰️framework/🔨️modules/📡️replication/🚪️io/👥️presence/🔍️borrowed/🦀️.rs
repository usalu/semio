//! 👥️ Borrowed projections validating the complete original presence wire.
use crate::wire::{PresencePeerWireLimitsV1, PRESENCE_PEER_WIRE_LIMITS_V1, PRESENCE_TYPING_EXCERPT_BYTES};
use crate::value::{ValueError, ValueRefusalKind};

fn invalid() -> ValueError { ValueError::literal(ValueRefusalKind::InvalidValue, "invalid original presence wire") }
fn limited() -> ValueError { ValueError::literal(ValueRefusalKind::WorkLimit, "original presence wire exceeds its declared limit") }

#[derive(Clone, Copy, Debug)]
pub struct BorrowedPresenceMetadata<'a> {
    pub actor: &'a str,
    pub connected_at_ms: i64,
    pub surface: Option<&'a str>,
    pub color: Option<u8>,
    pub presence_pack: Option<&'a [u8]>,
    pub interaction: Option<BorrowedPresenceInteraction<'a>>,
}

#[derive(Clone, Copy, Debug)]
pub struct BorrowedPresenceInteraction<'a> { pub app_id: &'a str, domains: &'a [u8], count: usize }
impl<'a> BorrowedPresenceInteraction<'a> {
    pub fn domains(self) -> BorrowedPresenceDomains<'a> { BorrowedPresenceDomains { reader: Reader::new(self.domains), remaining: self.count } }
}
#[derive(Clone, Copy, Debug)]
pub struct BorrowedPresenceDomain<'a> { pub domain: &'a str, pub granularity: &'a str, pub selected: BorrowedPresenceStrings<'a>, pub hovered: BorrowedPresenceStrings<'a> }
#[derive(Clone, Copy, Debug)]
pub struct BorrowedPresenceStrings<'a> { bytes: &'a [u8], count: usize }
impl<'a> BorrowedPresenceStrings<'a> {
    pub fn iter(self) -> BorrowedPresenceStringIter<'a> { BorrowedPresenceStringIter { reader: Reader::new(self.bytes), remaining: self.count } }
}
pub struct BorrowedPresenceStringIter<'a> { reader: Reader<'a>, remaining: usize }
impl<'a> Iterator for BorrowedPresenceStringIter<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 { return None; }
        self.remaining -= 1;
        Some(self.reader.text().expect("immutable validated original presence text"))
    }
}
pub struct BorrowedPresenceDomains<'a> { reader: Reader<'a>, remaining: usize }
impl<'a> Iterator for BorrowedPresenceDomains<'a> {
    type Item = BorrowedPresenceDomain<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 { return None; }
        self.remaining -= 1;
        Some(self.reader.domain().expect("immutable validated original presence domain"))
    }
}
struct Reader<'a> { bytes: &'a [u8], position: usize, limits: PresencePeerWireLimitsV1 }
impl<'a> Reader<'a> {
    fn new(bytes: &'a [u8]) -> Self { Self { bytes, position: 0, limits: PRESENCE_PEER_WIRE_LIMITS_V1 } }
    fn byte(&mut self) -> Result<u8, ValueError> {
        let value = self.bytes.get(self.position).copied().ok_or_else(invalid)?;
        self.position += 1; Ok(value)
    }
    fn varint(&mut self) -> Result<u64, ValueError> {
        let mut value = 0u64;
        for index in 0..10 {
            let byte = self.byte()?;
            if index == 9 && byte > 1 { return Err(invalid()); }
            value |= u64::from(byte & 0x7f) << (index * 7);
            if byte & 0x80 == 0 {
                if index > 0 && byte == 0 { return Err(invalid()); }
                return Ok(value);
            }
        }
        Err(invalid())
    }
    fn count(&mut self, maximum: usize) -> Result<usize, ValueError> {
        let value = self.varint()?;
        if value > maximum as u64 { return Err(limited()); }
        let value = usize::try_from(value).map_err(|_| limited())?;
        if value > self.bytes.len().saturating_sub(self.position) { return Err(invalid()); }
        Ok(value)
    }
    fn blob(&mut self, maximum: usize) -> Result<&'a [u8], ValueError> {
        let length = self.count(maximum)?;
        let result = &self.bytes[self.position..self.position + length];
        self.position += length; Ok(result)
    }
    fn text(&mut self) -> Result<&'a str, ValueError> { std::str::from_utf8(self.blob(self.limits.maximum_text_bytes)?).map_err(|_| invalid()) }
    fn boolean(&mut self) -> Result<bool, ValueError> { match self.byte()? { 0 => Ok(false), 1 => Ok(true), _ => Err(invalid()) } }
    fn number(&mut self) -> Result<(), ValueError> {
        let end = self.position.checked_add(8).ok_or_else(invalid)?;
        let bytes = self.bytes.get(self.position..end).ok_or_else(invalid)?;
        let value = f64::from_le_bytes(bytes.try_into().map_err(|_| invalid())?);
        if !value.is_finite() { return Err(invalid()); }
        self.position = end; Ok(())
    }
    fn numbers(&mut self, count: usize) -> Result<(), ValueError> { for _ in 0..count { self.number()?; } Ok(()) }
    fn strings(&mut self) -> Result<BorrowedPresenceStrings<'a>, ValueError> {
        let count = self.count(self.limits.maximum_domain_ids)?;
        let start = self.position;
        for _ in 0..count { self.text()?; }
        Ok(BorrowedPresenceStrings { bytes: &self.bytes[start..self.position], count })
    }
    fn domain(&mut self) -> Result<BorrowedPresenceDomain<'a>, ValueError> { Ok(BorrowedPresenceDomain { domain: self.text()?, granularity: self.text()?, selected: self.strings()?, hovered: self.strings()? }) }
    fn interaction(&mut self) -> Result<BorrowedPresenceInteraction<'a>, ValueError> {
        let app_id = self.text()?;
        let count = self.count(self.limits.maximum_interaction_domains)?;
        let start = self.position;
        for _ in 0..count { self.domain()?; }
        Ok(BorrowedPresenceInteraction { app_id, domains: &self.bytes[start..self.position], count })
    }
    fn views(&mut self) -> Result<(), ValueError> {
        let count = self.count(self.limits.maximum_views)?;
        for _ in 0..count {
            self.text()?; self.text()?;
            let numbers = match self.byte()? { 0 => 3, 1 => 10, 2 => 5, _ => return Err(invalid()) };
            self.numbers(numbers + 2)?;
            if self.boolean()? { self.numbers(3)?; }
            if self.boolean()? { self.numbers(3)?; }
        }
        Ok(())
    }
    fn optional_text(&mut self) -> Result<(), ValueError> { if self.boolean()? { self.text()?; } Ok(()) }
    fn units(&mut self) -> Result<u64, ValueError> { let value = self.varint()?; if value > self.limits.maximum_tool_run_units { Err(limited()) } else { Ok(value) } }
    fn tool_run(&mut self) -> Result<(), ValueError> {
        self.text()?;
        if usize::from(self.byte()?) >= crate::wire::PresenceToolRunState::ALL.len() { return Err(invalid()); }
        u16::try_from(self.varint()?).map_err(|_| limited())?;
        let completed = self.units()?;
        if self.boolean()? && completed > self.units()? { return Err(invalid()); }
        Ok(())
    }
    fn history_edit(&mut self) -> Result<(), ValueError> {
        self.text()?;
        if usize::from(self.byte()?) >= crate::wire::PresenceHistoryEditStage::ALL.len() { return Err(invalid()); }
        u32::try_from(self.varint()?).map_err(|_| limited())?;
        Ok(())
    }
    fn typing(&mut self) -> Result<(), ValueError> {
        let count = self.count(self.limits.maximum_typing_runs)?;
        if count == 0 { return Err(invalid()); }
        for _ in 0..count {
            self.text()?;
            if self.text()?.len() > PRESENCE_TYPING_EXCERPT_BYTES || self.text()?.len() > PRESENCE_TYPING_EXCERPT_BYTES { return Err(limited()); }
        }
        Ok(())
    }
}

/// 👥️ Validates every original field while exposing only borrowed metadata spans.
pub fn borrow_presence_metadata(bytes: &[u8]) -> Result<BorrowedPresenceMetadata<'_>, ValueError> {
    if bytes.len() > PRESENCE_PEER_WIRE_LIMITS_V1.maximum_entry_bytes { return Err(limited()); }
    let mut reader = Reader::new(bytes);
    let actor = reader.text()?;
    let flags = reader.varint()?;
    if flags >> 15 != 0 { return Err(invalid()); }
    let connected = reader.varint()?;
    if connected > reader.limits.maximum_connected_at_ms { return Err(limited()); }
    if flags & 1 != 0 { reader.text()?; }
    let presence_pack = if flags & (1 << 1) != 0 { Some(reader.blob(reader.limits.maximum_presence_pack_bytes)?) } else { None };
    for bit in [2, 3, 4] { if flags & (1 << bit) != 0 { reader.text()?; } }
    let interaction = if flags & (1 << 5) != 0 { Some(reader.interaction()?) } else { None };
    let color = if flags & (1 << 6) != 0 { Some(reader.byte()?) } else { None };
    let surface = if flags & (1 << 7) != 0 { Some(reader.text()?) } else { None };
    if flags & (1 << 8) != 0 { reader.views()?; }
    if flags & (1 << 9) != 0 { for _ in 0..3 { reader.optional_text()?; } }
    if flags & (1 << 10) != 0 { reader.tool_run()?; }
    if flags & (1 << 11) != 0 && usize::from(reader.byte()?) >= crate::wire::PresencePrincipalKind::ALL.len() { return Err(invalid()); }
    if flags & (1 << 12) != 0 { reader.text()?; }
    if flags & (1 << 13) != 0 { reader.history_edit()?; }
    if flags & (1 << 14) != 0 { reader.typing()?; }
    if reader.position != bytes.len() { return Err(invalid()); }
    Ok(BorrowedPresenceMetadata { actor, connected_at_ms: connected as i64, surface, color, presence_pack, interaction })
}
