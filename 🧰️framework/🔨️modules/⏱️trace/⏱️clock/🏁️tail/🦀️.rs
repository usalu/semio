//! 🏁️ Preserves one original watchdog across admission, publication and optional telemetry.
use super::{try_now_us, CallbackClockFault, CallbackVerdict, Watchdog};

//#region 🐕️Admission
/// 🎟️ Affine intermediate owner; diagnostic verdicts cannot construct or replace its guard.
#[must_use = "Finish the original guard into the retained callback owner."]
pub struct WatchdogAdmission {
    guard: Watchdog,
    admission: CallbackVerdict,
    observed_us: Option<u64>,
}

impl Watchdog {
    /// 🚦️ Checks publication admission without finishing or restarting the original window.
    pub fn admission_checkpoint(self) -> WatchdogAdmission {
        let observed_us = try_now_us();
        let admission = self.verdict_at(observed_us);
        WatchdogAdmission { guard: self, admission, observed_us }
    }
}

impl WatchdogAdmission {
    /// 🔎️ Returns diagnostic data, never a constructible commit capability.
    pub fn verdict(&self) -> CallbackVerdict { self.admission }

    /// 🏁️ Records interim optional telemetry before the terminal reading of the same window.
    pub fn finish_after_telemetry(mut self) -> CallbackVerdict {
        self.guard.finished = true;
        let sample_us = try_now_us();
        let fault = self.admission.clock_fault().or_else(|| clock_order_fault(self.observed_us, sample_us));
        self.guard.report(sample_us);
        let terminal_us = try_now_us();
        let fault = fault.or_else(|| clock_order_fault(sample_us, terminal_us));
        let mut terminal = self.guard.verdict_at(terminal_us);
        if let Some(fault) = fault { terminal.elapsed = Err(fault); }
        terminal
    }
}

fn clock_order_fault(before: Option<u64>, after: Option<u64>) -> Option<CallbackClockFault> {
    match (before, after) {
        (Some(before), Some(after)) if after >= before => None,
        (Some(_), Some(_)) => Some(CallbackClockFault::Backward),
        _ => Some(CallbackClockFault::Missing),
    }
}
//#endregion 🐕️Admission

