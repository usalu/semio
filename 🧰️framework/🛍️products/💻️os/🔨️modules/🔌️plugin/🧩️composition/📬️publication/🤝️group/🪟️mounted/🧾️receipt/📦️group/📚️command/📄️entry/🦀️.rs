use super::*;

const ENTRY_LABELS: usize = Terminology::COUNT * Locale::COUNT;
const ENTRY_FIELDS: usize = ENTRY_LABELS + 2;

#[derive(Clone, Copy)]
pub(crate) struct MountedCommandEntrySource<'a> {
    pub(crate) action_id: &'a str,
    pub(crate) label: Option<&'a LocalizedLabel>,
    pub(crate) captured_millis: i64,
    pub(crate) kind: ActionKind,
    pub(crate) count: u32,
    pub(crate) parent_touched: bool,
    pub(crate) child_edit_count: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum MountedCommandEntryRefusal { Timestamp, Stale, Closed }

impl<'a> MountedCommandEntrySource<'a> {
    fn text(&self, index: usize) -> &'a str {
        if index == 0 { self.action_id } else { self.label.map(|label| label.resolve(Terminology::ALL[(index - 1) / Locale::COUNT], Locale::ALL[(index - 1) % Locale::COUNT])).unwrap_or(self.action_id) }
    }
    fn seal(&self) -> [(usize, usize); ENTRY_LABELS + 1] { std::array::from_fn(|index| { let text = self.text(index); (text.as_ptr() as usize, text.len()) }) }
}

pub(crate) struct MountedCommandEntry {
    fields: ManuallyDrop<[Option<String>; ENTRY_FIELDS]>,
    active: ManuallyDrop<Option<Vec<u8>>>,
    source: [(usize, usize); ENTRY_LABELS + 1],
    label_identity: Option<usize>,
    captured_millis: i64,
    timestamp: [u8; 24],
    kind: ActionKind,
    count: u32,
    parent_touched: bool,
    child_edit_count: usize,
    field: usize,
    closing: bool,
}

impl MountedCommandEntry {
    /// 🕰️ Seals the borrowed action, every label cell and the single UTC capture without allocation.
    pub(crate) fn new(source: MountedCommandEntrySource<'_>) -> Result<Self, MountedCommandEntryRefusal> {
        let timestamp = Self::utc_iso(source.captured_millis)?;
        Ok(Self { fields: ManuallyDrop::new(std::array::from_fn(|_| None)), active: ManuallyDrop::new(None), source: source.seal(), label_identity: source.label.map(|label| label as *const LocalizedLabel as usize), captured_millis: source.captured_millis, timestamp, kind: source.kind, count: source.count, parent_touched: source.parent_touched, child_edit_count: source.child_edit_count, field: 0, closing: false })
    }
    fn validate(&self, source: MountedCommandEntrySource<'_>) -> Result<(), MountedCommandEntryRefusal> {
        if self.closing { return Err(MountedCommandEntryRefusal::Closed); }
        if self.source != source.seal() || self.label_identity != source.label.map(|label| label as *const LocalizedLabel as usize) || self.captured_millis != source.captured_millis || self.kind != source.kind || self.count != source.count || self.parent_touched != source.parent_touched || self.child_edit_count != source.child_edit_count { return Err(MountedCommandEntryRefusal::Stale); }
        Ok(())
    }
    fn bytes<'a>(&'a self, source: MountedCommandEntrySource<'a>) -> &'a [u8] { if self.field == ENTRY_FIELDS - 1 { &self.timestamp } else { source.text(self.field).as_bytes() } }
    pub(crate) fn ready(&self) -> bool { !self.closing && self.field == ENTRY_FIELDS }
    pub(crate) fn next_capacity_byte_demand(&self, source: MountedCommandEntrySource<'_>) -> Result<usize, MountedCommandEntryRefusal> { self.validate(source)?; Ok(if self.ready() || self.active.is_some() { 0 } else { self.bytes(source).len() }) }
    pub(crate) fn next_minimum_copy_byte_demand(&self) -> usize { usize::from(self.active.as_ref().is_some_and(|active| self.field != ENTRY_FIELDS && active.len() < if self.field == ENTRY_FIELDS - 1 { self.timestamp.len() } else { self.source[self.field].1 })) }
    /// 🧵️ Births one whole field, then copies at most the admitted 64-byte borrowed prefix.
    pub(crate) fn advance(&mut self, source: MountedCommandEntrySource<'_>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, MountedCommandEntryRefusal> {
        self.validate(source)?;
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if self.ready() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        let length = self.bytes(source).len();
        if self.active.is_none() {
            if grant.maximum_capacity_bytes < length { return Ok(RetainedCloneStep::Progress(Default::default())); }
            *self.active = Some(Vec::with_capacity(length));
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, retained_capacity_bytes: length, ..Default::default() }));
        }
        let offset = self.active.as_ref().unwrap().len();
        let copied = (length - offset).min(grant.maximum_copy_bytes).min(64);
        if copied == 0 && offset != length { return Ok(RetainedCloneStep::Progress(Default::default())); }
        let bytes = if self.field == ENTRY_FIELDS - 1 { &self.timestamp[..] } else { source.text(self.field).as_bytes() };
        self.active.as_mut().unwrap().extend_from_slice(&bytes[offset..offset + copied]);
        if offset + copied == length { self.fields[self.field] = Some(unsafe { String::from_utf8_unchecked(self.active.take().unwrap()) }); self.field += 1; }
        Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: copied, ..Default::default() }))
    }
    /// 🧳️ Uses the final append sequence and moves the original canonical receipt IDs without copying.
    pub(crate) fn take(&mut self, parent: &mut Option<String>, children: &mut Vec<String>, seq: u64, grant: RetainedCloneGrant) -> Option<CommandLogEntry> {
        if !self.ready() || grant.maximum_items == 0 || parent.is_some() != self.parent_touched || children.len() != self.child_edit_count { return None; }
        let action_id = self.fields[0].take().unwrap();
        let mut label = LocalizedLabel::default();
        for (index, cell) in label.texts_mut().enumerate() { *cell = self.fields[index + 1].take().unwrap(); }
        let timestamp = self.fields[ENTRY_FIELDS - 1].take().unwrap();
        self.closing = true;
        Some(CommandLogEntry { seq, action_id, label, kind: self.kind, timestamp, edit_id: parent.take(), child_edit_ids: std::mem::take(children), transition_id: None, count: self.count, inverse: None })
    }
    pub(crate) fn next_close_byte_demand(&self) -> usize { self.active.as_ref().map(Vec::capacity).or_else(|| self.fields.iter().flatten().next().map(String::capacity)).unwrap_or(0) }
    pub(crate) fn terminal_is_empty(&self) -> bool { self.active.is_none() && self.fields.iter().all(Option::is_none) }
    /// 🪵️ Releases one original field allocation only with its whole same-turn physical authority.
    pub(crate) fn close_step(&mut self, grant: RetainedCloneGrant) -> RetainedCloneStep {
        if self.terminal_is_empty() { self.closing = true; return RetainedCloneStep::Complete(Default::default()); }
        if grant.maximum_items == 0 || grant.maximum_release_bytes < self.next_close_byte_demand() { return RetainedCloneStep::Progress(Default::default()); }
        self.closing = true;
        let bytes = self.next_close_byte_demand();
        if self.active.is_some() { drop(self.active.take()); } else { drop(self.fields.iter_mut().find(|field| field.is_some()).unwrap().take()); }
        RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: bytes, ..Default::default() })
    }
    fn utc_iso(millis: i64) -> Result<[u8; 24], MountedCommandEntryRefusal> {
        if !(-62_167_219_200_000..=253_402_300_799_999).contains(&millis) { return Err(MountedCommandEntryRefusal::Timestamp); }
        let z = millis.div_euclid(86_400_000) + 719_468;
        let era = z.div_euclid(146_097);
        let doe = z - era * 146_097;
        let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
        let mut year = yoe + era * 400;
        let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
        let mp = (5 * doy + 2) / 153;
        let day = doy - (153 * mp + 2) / 5 + 1;
        let month = mp + if mp < 10 { 3 } else { -9 };
        year += i64::from(month <= 2);
        let time = millis.rem_euclid(86_400_000);
        let mut output = *b"0000-00-00T00:00:00.000Z";
        for (start, width, mut value) in [(0,4,year),(5,2,month),(8,2,day),(11,2,time/3_600_000),(14,2,time/60_000%60),(17,2,time/1000%60),(20,3,time%1000)] { for index in (start..start + width).rev() { output[index] = b'0' + (value % 10) as u8; value /= 10; } }
        Ok(output)
    }
}

impl Drop for MountedCommandEntry { fn drop(&mut self) { assert!(std::thread::panicking() || self.terminal_is_empty(), "preborn command fields require canonical ID transfer or whole funded close"); } }

#[cfg(test)]
include!("🧪️tests/🦀️.rs");
