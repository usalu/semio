//! 👥️ Presence: the bounded roster wire and the paged presence-command cursor.

use ::protocol::wire::command_ingress::{FixedCommandPage, COMMAND_PAGE_MAXIMUM_BYTES};
use super::malformed;

//#region 🔖️PresenceRosterWire
pub const PRESENCE_ROSTER_MAXIMUM_ITEMS: usize = 64;
pub const PRESENCE_ROSTER_MAXIMUM_ENTRY_BYTES: usize = 4_096;
pub const PRESENCE_ROSTER_MAXIMUM_BYTES: usize = PRESENCE_ROSTER_MAXIMUM_ITEMS * PRESENCE_ROSTER_MAXIMUM_ENTRY_BYTES;

#[derive(Debug, PartialEq)]
pub struct PresenceRosterWire {
    pub(super) entries: [Option<Box<[u8]>>; PRESENCE_ROSTER_MAXIMUM_ITEMS],
    len: usize,
    pub(super) bytes: usize,
}

#[derive(Debug)]
pub struct PresenceRosterPushRejected {
    pub entry: Vec<u8>,
    pub reason: &'static str,
}

impl PresenceRosterWire {
    pub fn empty() -> Self {
        Self { entries: std::array::from_fn(|_| None), len: 0, bytes: 0 }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn byte_len(&self) -> usize {
        self.bytes
    }

    pub fn try_push(&mut self, entry: Vec<u8>) -> Result<(), PresenceRosterPushRejected> {
        if self.len == PRESENCE_ROSTER_MAXIMUM_ITEMS {
            return Err(PresenceRosterPushRejected { entry, reason: "presence roster exceeds its fixed item authority" });
        }
        if entry.len() > PRESENCE_ROSTER_MAXIMUM_ENTRY_BYTES || self.bytes.checked_add(entry.len()).is_none_or(|bytes| bytes > PRESENCE_ROSTER_MAXIMUM_BYTES) {
            return Err(PresenceRosterPushRejected { entry, reason: "presence roster exceeds its fixed byte authority" });
        }
        let bytes = entry.len();
        self.entries[self.len] = Some(entry.into_boxed_slice());
        self.len += 1;
        self.bytes += bytes;
        Ok(())
    }

    pub fn pop_back(&mut self) -> Option<Box<[u8]>> {
        if self.len == 0 {
            return None;
        }
        self.len -= 1;
        let entry = self.entries[self.len].take().expect("occupied presence roster slot");
        self.bytes -= entry.len();
        Some(entry)
    }

    pub fn iter(&self) -> impl Iterator<Item = &[u8]> {
        self.entries[..self.len].iter().filter_map(|entry| entry.as_deref())
    }

    pub fn last(&self) -> Option<&[u8]> {
        self.len.checked_sub(1).and_then(|index| self.entries[index].as_deref())
    }
}

impl Default for PresenceRosterWire {
    fn default() -> Self {
        Self::empty()
    }
}
//#endregion 🔖️PresenceRosterWire

//#region 🔖️PresenceCommandCursor
pub const PRESENCE_COMMAND_MAXIMUM_BYTES: usize = PRESENCE_ROSTER_MAXIMUM_BYTES + PRESENCE_ROSTER_MAXIMUM_ITEMS * 10 + 32;

pub struct PresenceCommandCursor {
    page: Option<FixedCommandPage>,
    pub(super) remaining: usize,
    pub(super) seq: u64,
    pub(super) own_color: Option<u8>,
    next_page: u32,
}

impl PresenceCommandCursor {
    pub fn retain_rejected(page: FixedCommandPage, seq: u64) -> Self {
        Self { page: Some(page), remaining: 0, seq, own_color: None, next_page: 1 }
    }

    pub fn admit_page(seq: u64, own_color: Option<u8>, item_count: u32, page: FixedCommandPage) -> Result<Self, (protocol::ProtocolError, FixedCommandPage)> {
        if item_count as usize > PRESENCE_ROSTER_MAXIMUM_ITEMS || page.len() > PRESENCE_ROSTER_MAXIMUM_ENTRY_BYTES || (item_count != 0 && page.is_empty()) || (item_count == 0 && !page.is_empty()) {
            return Err((malformed("channel presence page", 0, "Presence page violates its exact item or 4096-byte authority"), page));
        }
        Ok(Self { page: Some(page), remaining: item_count as usize, seq, own_color, next_page: 1 })
    }

    pub fn push_page(&mut self, page_index: u32, page: FixedCommandPage) -> Result<(), (protocol::ProtocolError, FixedCommandPage)> {
        if self.page.is_some() || self.remaining == 0 || page_index != self.next_page || page.is_empty() || page.len() > PRESENCE_ROSTER_MAXIMUM_ENTRY_BYTES {
            return Err((malformed("channel presence page", page_index as u64, "page is out of order, saturated, empty, or oversized"), page));
        }
        self.page = Some(page);
        self.next_page = self.next_page.saturating_add(1);
        Ok(())
    }

    pub fn seq(&self) -> u64 {
        self.seq
    }

    pub fn own_color(&self) -> Option<u8> {
        self.own_color
    }

    pub fn take_next(&mut self) -> Result<Option<FixedCommandPage>, protocol::ProtocolError> {
        if self.remaining == 0 {
            return Ok(None);
        }
        let entry = self.page.take().ok_or_else(|| malformed("channel presence command owner", self.next_page as u64, "next page has not been admitted"))?;
        self.remaining -= 1;
        Ok(Some(entry))
    }

    pub fn next_len(&self) -> Result<Option<usize>, protocol::ProtocolError> {
        if self.remaining == 0 {
            return Ok(None);
        }
        Ok(self.page.as_ref().map(FixedCommandPage::len))
    }

    /// ♻️ Releases the exact original fixed-page backing only after all independent axes admit it.
    pub fn close_step(&mut self, grant: semio_framework_value::RetainedCloneGrant) -> semio_framework_value::RetainedCloneStep {
        use semio_framework_value::{RetainedCloneProgress, RetainedCloneStep};
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() { return RetainedCloneStep::Complete(empty); }
        if grant.maximum_items == 0 || grant.maximum_depth == 0 { return RetainedCloneStep::Progress(empty); }
        let copy = size_of::<FixedCommandPage>();
        let release = if self.page.is_some() { COMMAND_PAGE_MAXIMUM_BYTES } else { 0 };
        if self.page.is_some() && grant.maximum_copy_bytes < copy || grant.maximum_release_bytes < release { return RetainedCloneStep::Progress(empty); }
        let moved = self.page.is_some();
        drop(self.page.take());
        self.remaining = 0;
        RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, copied_bytes: usize::from(moved) * copy, released_bytes: release, ..empty })
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.page.is_none() && self.remaining == 0
    }

    pub fn waiting_for_page(&self) -> bool {
        self.page.is_none() && self.remaining != 0
    }
}
//#endregion 🔖️PresenceCommandCursor
