//! 🔦️ The named more-work sources of one reactor turn and their settled reading.

/// 🔦️ The named sources a turn folds into `TurnStatus::MoreWork`, recorded once per turn.
///
/// An idle actor must answer `Idle`: every `MoreWork` costs the host another turn round trip, and at
/// the browser's cadence that is the difference between a settled app and a permanently hot reactor.
/// A law that asserts "idle" needs to say WHICH source stayed armed, so the reading is structural
/// rather than a boolean. Costs one `Cell` store per turn. See `📓️idle-turns-2026-09-10.md`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TurnMoreWorkSources {
    /// ⏱️ The reactor executor still had ready work when its 8 ms slice expired.
    pub executor_deadline: bool,
    /// 🧵️ The cooperative process-worker pool advanced a worker this turn.
    pub process_pool: bool,
    /// 🚪️ An instance close ladder has un-run cleanup steps.
    pub close_cleanup: bool,
    /// 🧬️ A typed operation continuation has not reached its terminal state.
    pub typed_operation: bool,
    /// 🔒️ OBSERVATION, never work: another owner held an instance's operation authority when the
    /// scan ran, so the scan could not answer. Deliberately outside [`Self::any`] — see the fold in
    /// `poll_kernel_turn`.
    pub typed_operation_contended: bool,
    /// ⏱️ An armed per-instance pending-effects owner answered `None` and owes its next paid poll.
    pub pending_effects: bool,
    /// 🩹️ A surface reconcile has publishable or unpublished patch work.
    pub reconcile: bool,
    /// 🔁️ Resumed async tasks remained undrained at the resume budget.
    pub resumes: bool,
    /// 🧵️ The reactor executor holds tasks that are parked, not ready.
    pub executor_pending: bool,
    /// 📥️ A command ingress slot still owns an in-assembly command.
    pub command_ingress: bool,
    /// 📨️ An original extension request still owes invocation, publication or paid retirement.
    pub extension_invocation: bool,
    /// 🚪️ The guest lifecycle registry owes a receipt or a transition.
    pub lifecycle: bool,
    /// ♻️ UI owners released during this turn still owe bounded retirement work.
    pub ui_retirement: bool,
    /// 🛬️ A cold-pair or checkpoint-restored document still loads through the stepped archive load.
    pub document_load: bool,
}

impl TurnMoreWorkSources {
    /// 😴️ The reading of a turn that answered `Idle`.
    pub const SETTLED: Self = Self {
        executor_deadline: false,
        process_pool: false,
        close_cleanup: false,
        typed_operation: false,
        typed_operation_contended: false,
        pending_effects: false,
        reconcile: false,
        resumes: false,
        executor_pending: false,
        command_ingress: false,
        extension_invocation: false,
        lifecycle: false,
        ui_retirement: false,
        document_load: false,
    };

    /// 🔦️ Whether any source is armed — equal to the turn's `MoreWork` verdict.
    pub fn any(self) -> bool {
        !self.names().is_empty()
    }

    /// 🏷️ The armed source names, for a law's failure message.
    pub fn names(self) -> Vec<&'static str> {
        [
            (self.executor_deadline, "executor_deadline"),
            (self.process_pool, "process_pool"),
            (self.close_cleanup, "close_cleanup"),
            (self.typed_operation, "typed_operation"),
            (self.pending_effects, "pending_effects"),
            (self.reconcile, "reconcile"),
            (self.resumes, "resumes"),
            (self.executor_pending, "executor_pending"),
            (self.command_ingress, "command_ingress"),
            (self.extension_invocation, "extension_invocation"),
            (self.lifecycle, "lifecycle"),
            (self.ui_retirement, "ui_retirement"),
            (self.document_load, "document_load"),
        ]
        .into_iter()
        .filter_map(|(armed, name)| armed.then_some(name))
        .collect()
    }
}

#[cfg(test)]
mod pending_effects_source_law {
    use super::TurnMoreWorkSources;
    #[test]
    fn retained_pending_effects_arm_a_named_more_work_source() {
        let armed = TurnMoreWorkSources { pending_effects: true, ..Default::default() };
        assert!(armed.any());
        assert_eq!(armed.names(), vec!["pending_effects"]);
        assert!(!TurnMoreWorkSources { typed_operation_contended: true, ..Default::default() }.any());
        assert!(!TurnMoreWorkSources::SETTLED.any());
    }
}
