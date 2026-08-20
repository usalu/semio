//! @emoji 🎯️ Intent routing, revision guarding, and `DispatchOutcome`.
//!
//! A [`HandleIntent`] handler is the one place a [`ui_contract::UiIntent`] turns into local mutation
//! plus a *description* of outward effects — it never submits a command or publishes presence
//! itself, it only tells `runtime-transact`'s `crate::UiRuntime::transact` what to do with them once
//! every queued intent this transaction has been routed, which is what keeps ordering well-defined
//! across a whole run-to-completion frame (ruling U1).
//!
//! **The revision guard is the point.** [`is_stale_intent`] is the pure predicate `transact` applies
//! before a queued intent ever reaches a [`HandleIntent::on_intent`] call: an intent whose
//! [`ui_contract::UiIntent::revision`] trails the surface's current revision by more than
//! [`DEFAULT_REVISION_TOLERANCE`] targets geometry the user never actually saw, so it is dropped —
//! never dispatched, never a patch, never a command. [`DispatchOutcome::Stale`] is the separate,
//! in-band signal a handler can *also* return for a staleness reason the revision guard alone could
//! not see (e.g. the targeted node has since become disabled) — both paths converge on "no patch, no
//! command".
//!
//! 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md. Every `fn`
//! below is plain sync by owner ruling U1, which supersedes this program's general async-everything
//! default for exactly this crate.

//#region 🔖️Intent

/// 🎯️ A run-to-completion, revision-guarded moment: a presenter's own type implements this once, and
/// `crate::UiRuntime::transact` routes every queued [`ui_contract::UiIntent`] addressed at that
/// presenter's registered surface through it. `cx` is the same lease-scoped `crate::Context` any
/// other entity mutation uses — `notify`/`emit`/`defer`/`spawn_local`, never an await, by the same
/// construction `crate::EntityStore::update` already enforces on every other mutation path.
pub trait HandleIntent: Sized {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn on_intent(&mut self, intent: &ui_contract::UiIntent, cx: &mut crate::Context<'_, Self>) -> DispatchOutcome;
}

/// 🎚️ The default revision-staleness tolerance [`is_stale_intent`] applies: an intent trailing the
/// surface's current revision by exactly this many revisions is still dispatched — the reconciler's
/// own patch may simply not have reached the renderer yet even though it was already computed — but
/// trailing by more is stale.
pub const DEFAULT_REVISION_TOLERANCE: u64 = 1;

/// 🚦️ `true` when `intent_revision` trails `current_revision` by MORE than `tolerance` — the pure
/// predicate behind the revision guard, kept free of any surface/store lookup so it is trivially unit
/// testable on its own. An `intent_revision` at or ahead of `current_revision` is never stale by this
/// predicate — a forward-dated intent is not a case this crate itself ever produces.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn is_stale_intent(intent_revision: ui_contract::UiRevision, current_revision: ui_contract::UiRevision, tolerance: u64) -> bool {
    current_revision.0.saturating_sub(intent_revision.0) > tolerance
}

//#endregion 🔖️Intent

//#region 🔖️Outcome

/// 📤️ What one [`HandleIntent::on_intent`] call produced. Never carries a future or performs a
/// side effect itself (ruling U1) — a handler *describes* outward effects instead, which is what lets
/// `crate::UiRuntime::transact` apply them in a well-defined order after every queued intent this
/// transaction has been routed.
#[derive(Debug)]
pub enum DispatchOutcome {
    /// ✅️ The intent was applied; no outward effect beyond whatever entity mutation the handler made
    /// (itself already reflected by the next present/reconcile pass).
    Handled,
    /// ✅️ The intent was applied and additionally describes commands to submit and/or deferred work
    /// for `transact` to carry out.
    HandledWith { commands: Vec<crate::Command>, deferred: Vec<DeferredOp> },
    /// 🕰️ Dropped: the handler itself recognizes this intent as targeting a stale moment, for a
    /// reason the revision guard alone could not see. Converges with a revision-guard drop: no patch,
    /// no command.
    Stale,
    /// ❔️ No binding on this presenter recognized `intent.action`/`intent.trigger` — dropped exactly
    /// like `Stale`, but distinguishable in a log or a metric.
    Unhandled,
}

/// ⏭️ One outward effect a [`DispatchOutcome::HandledWith`] describes instead of performing inline.
#[derive(Debug)]
pub enum DeferredOp {
    /// 📤️ Submit through `crate::UiRuntime`'s own `CommandGateway` — the same non-blocking, bounded
    /// path any other command takes.
    SubmitCommand(crate::Command),
    /// 👥️ Record onto `crate::UiRuntime`'s own `PresenceHub`, decomposed into `record_own`/
    /// `record_peer` calls — presence never enters a document revision (see `🦀️presence.rs`'s module
    /// doc), so it never becomes a `UiPatch` op either.
    PublishPresence(ui_contract::PresenceUpdate),
    /// 🧩️ Anything this crate has no built-in interpretation for — looked up by key in whatever
    /// fn-pointer `crate::UiRuntime::register_custom_deferred` registered for it (never `dyn Fn`,
    /// ruling U3). A key nobody registered a handler for is a safe no-op, never a panic: a handler
    /// describing effects must never be able to crash the transaction that carries them out.
    Custom(DeferredKey),
}

/// 🔑️ An opaque tag naming a [`DeferredOp::Custom`] effect — matched by value against whatever
/// fn-pointer `crate::UiRuntime::register_custom_deferred` was given for the same key.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DeferredKey(pub &'static str);

//#endregion 🔖️Outcome

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_intent_at_or_ahead_of_the_current_revision_is_never_stale() {
        assert!(!is_stale_intent(ui_contract::UiRevision(5), ui_contract::UiRevision(5), DEFAULT_REVISION_TOLERANCE));
        assert!(!is_stale_intent(ui_contract::UiRevision(9), ui_contract::UiRevision(5), DEFAULT_REVISION_TOLERANCE));
    }

    #[test]
    fn an_intent_exactly_at_the_tolerance_is_not_yet_stale() {
        assert!(!is_stale_intent(ui_contract::UiRevision(4), ui_contract::UiRevision(5), 1));
    }

    #[test]
    fn an_intent_trailing_by_more_than_the_tolerance_is_stale() {
        assert!(is_stale_intent(ui_contract::UiRevision(3), ui_contract::UiRevision(5), 1));
    }

    #[test]
    fn a_zero_tolerance_makes_any_trailing_revision_stale() {
        assert!(is_stale_intent(ui_contract::UiRevision(4), ui_contract::UiRevision(5), 0));
        assert!(!is_stale_intent(ui_contract::UiRevision(5), ui_contract::UiRevision(5), 0));
    }
}
//#endregion 🧪️Tests
