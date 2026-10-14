//! 📦️ One original issued batch owns cumulative retained authority across borrowed envelope recipients.
use super::{RetainedTurnInput,RetainedTurnReceipt};
use semio_framework_value::{ValueError,ValueRefusalKind,RetainedCloneGrant,RetainedCloneProgress};

#[derive(Debug, PartialEq)]
pub struct IssuedRetainedBatch {
    original: RetainedTurnInput,
    envelopes: usize,
    next: usize,
    issued: Option<RetainedTurnInput>,
    spent: RetainedCloneProgress,
    active_progress: RetainedCloneProgress,
    returned: bool,
}

impl IssuedRetainedBatch {
    pub fn new(original: RetainedTurnInput, envelopes: usize) -> Result<Self, ValueError> {
        original.validate()?;
        if envelopes == 0 { return Err(Self::refusal("issued shard batch requires an original envelope")); }
        Ok(Self { original, envelopes, next: 0, issued: None, spent: Default::default(), active_progress: Default::default(), returned: false })
    }
    pub fn original_input(&self) -> &RetainedTurnInput { &self.original }
    pub fn remaining_input(&self) -> Result<RetainedTurnInput, ValueError> {
        if self.returned { return Err(Self::refusal("issued shard batch has already returned")); }
        Ok(RetainedTurnInput { grant: self.original_input().return_original(self.spent.checked_add(self.active_progress)?)?.remaining, ..*self.original_input() })
    }
    pub fn issue_envelope(&mut self, index: usize) -> Result<RetainedTurnInput, ValueError> {
        if index != self.next || index >= self.envelopes || self.issued.is_some() { return Err(Self::refusal("issued shard envelope is absent, duplicated, or out of order")); }
        let input = self.remaining_input()?;
        self.issued = Some(input);
        Ok(input)
    }
    pub fn settle_envelope(&mut self, index: usize, receipt: RetainedTurnReceipt) -> Result<Option<RetainedTurnReceipt>, ValueError> {
        if self.returned || index != self.next || index >= self.envelopes { return Err(Self::refusal("issued shard envelope cannot complete twice or out of order")); }
        let issued = self.issued.ok_or_else(|| Self::refusal("issued shard envelope has no original checkout"))?;
        receipt.validate_for(issued)?;
        let active = self.active_progress;
        if receipt.spent.copied_items < active.copied_items || receipt.spent.copied_bytes < active.copied_bytes || receipt.spent.retained_capacity_bytes < active.retained_capacity_bytes || receipt.spent.released_bytes < active.released_bytes { return Err(Self::refusal("issued shard receipt omits already received physical spans")); }
        let spent = self.spent.checked_add(receipt.spent)?;
        let whole = self.original_input().return_original(spent)?;
        let next = self.next.checked_add(1).ok_or_else(|| Self::refusal("issued shard envelope index overflow"))?;
        self.spent = spent; self.active_progress = Default::default(); self.issued = None; self.next = next;
        if next == self.envelopes { self.returned = true; Ok(Some(whole)) } else { Ok(None) }
    }
    pub fn envelope_recipient(&mut self, index: usize) -> Result<IssuedRetainedRecipient<'_>, ValueError> {
        let input = self.issue_envelope(index)?;
        Ok(IssuedRetainedRecipient { turn: self, index, input, finished: false })
    }
    /// 🔁️ Reborrows the same outstanding checkout and its already accepted physical spans.
    pub fn resume_envelope_recipient(&mut self, index: usize) -> Result<IssuedRetainedRecipient<'_>, ValueError> {
        if self.returned || index != self.next || index >= self.envelopes { return Err(Self::refusal("issued batch has no matching suspended envelope")); }
        let input = self.issued.ok_or_else(|| Self::refusal("issued batch has no outstanding original checkout"))?;
        Ok(IssuedRetainedRecipient { turn: self, index, input, finished: false })
    }
    pub fn refusal(message: &'static str) -> ValueError {
        ValueError::literal(ValueRefusalKind::InvariantViolated, message)
    }
}

/// 🫴️ Borrows one issued envelope and settles its actual spans into the original batch.
pub struct IssuedRetainedRecipient<'a> {
    turn: &'a mut IssuedRetainedBatch,
    index: usize,
    input: RetainedTurnInput,
    finished: bool,
}
impl IssuedRetainedRecipient<'_> {
    /// 👁️ Borrows the unchanged issued envelope identity and its original remaining currencies.
    pub fn original_input(&self) -> &RetainedTurnInput { &self.input }
    /// 🧮️ Returns only authority unspent by accepted spans in this envelope.
    pub fn parent_remaining_input(&self) -> Result<RetainedTurnInput, ValueError> { self.turn.remaining_input() }
    pub fn settled_progress(&self) -> RetainedCloneProgress { self.turn.spent }
    pub fn remaining_grant(&self) -> Result<RetainedCloneGrant, ValueError> {
        if self.finished { return Err(IssuedRetainedBatch::refusal("issued recipient has already settled")); }
        Ok(self.input.return_original(self.turn.active_progress)?.remaining)
    }
    /// 🧾️ Retains the actual child delta before reporting an exceeded original grant.
    pub fn record_progress(&mut self, progress: RetainedCloneProgress) -> Result<(), ValueError> {
        if self.finished { return Err(IssuedRetainedBatch::refusal("issued shard recipient has already settled")); }
        let next = self.turn.active_progress.checked_add(progress)?;
        self.turn.active_progress = next;
        self.input.return_original(next).map(|_| ())
    }
    /// 📬️ Validates one envelope return and emits a parent receipt only at batch completion.
    pub fn finish(&mut self) -> Result<Option<RetainedTurnReceipt>, ValueError> {
        if self.finished { return Err(IssuedRetainedBatch::refusal("issued shard recipient cannot return twice")); }
        let receipt = self.input.return_original(self.turn.active_progress)?;
        let whole = self.turn.settle_envelope(self.index, receipt)?;
        self.finished = true;
        Ok(whole)
    }
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
