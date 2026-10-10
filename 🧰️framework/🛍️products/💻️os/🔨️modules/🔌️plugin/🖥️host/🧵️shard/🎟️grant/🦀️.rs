//! 🎟️ Original scheduler decision custody across checked native frame publication.

use semio_framework_actor::{pack::PackError, Decision, TurnGrant};

/// ⚖️ The caller may override scheduling resources without receiving original authority.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ShardResourceBudget {
    pub fuel: u64,
    pub wall_ms: u32,
    pub memory_bytes: u64,
    pub ui_nodes: u32,
    pub mailbox_len: u16,
    pub max_effects: u32,
    pub max_patch_bytes: u32,
}
impl ShardResourceBudget {
    pub fn from_original(budget: &semio_framework_actor::Budget) -> Self {
        Self { fuel: budget.fuel, wall_ms: budget.wall_ms, memory_bytes: budget.memory_bytes, ui_nodes: budget.ui_nodes, mailbox_len: budget.mailbox_len, max_effects: budget.max_effects, max_patch_bytes: budget.max_patch_bytes }
    }
    pub fn apply(self, budget: &mut semio_framework_actor::Budget) {
        budget.fuel = self.fuel; budget.wall_ms = self.wall_ms; budget.memory_bytes = self.memory_bytes;
        budget.ui_nodes = self.ui_nodes; budget.mailbox_len = self.mailbox_len; budget.max_effects = self.max_effects; budget.max_patch_bytes = self.max_patch_bytes;
    }
}

#[derive(Default)]
pub struct OriginalShardDispatch {
    decision: Option<Decision>,
    cursor: usize,
    refusal: Option<PackError>,
}

impl OriginalShardDispatch {
    pub fn decision(&self) -> Option<&Decision> { self.decision.as_ref() }
    pub fn refusal(&self) -> Option<&PackError> { self.refusal.as_ref() }
    pub fn cursor(&self) -> usize { self.cursor }

    pub fn admit(&mut self, decision: &mut Option<Decision>) -> Result<(), PackError> {
        if let Some(error) = self.refusal { return Err(error); }
        if self.decision.is_some() { return Ok(()); }
        self.decision = decision.take();
        if let Some(decision) = &self.decision {
            for grant in &decision.run {
                if grant.original_input().validate().is_err() {
                    let error = PackError::InvalidRetainedTurn("operation identity is absent");
                    self.refusal = Some(error);
                    return Err(error);
                }
            }
        }
        Ok(())
    }

    pub fn next(&self) -> Option<&TurnGrant> { self.decision.as_ref()?.run.get(self.cursor) }
    pub fn published(&mut self) { self.cursor += 1; }
    pub fn refuse(&mut self, error: PackError) -> PackError { self.refusal = Some(error); error }

    pub fn finish(&mut self) -> Option<Decision> {
        if self.refusal.is_some() || self.next().is_some() { return None; }
        self.cursor = 0;
        self.decision.take()
    }
}
