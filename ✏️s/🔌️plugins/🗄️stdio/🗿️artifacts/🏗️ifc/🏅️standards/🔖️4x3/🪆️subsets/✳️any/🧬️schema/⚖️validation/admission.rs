//! 🛂 Original semantic admission owns real graph preparation and never substitutes cold evaluation for retained execution.
use super::{overlay::*, receive::PopulationReceiver, retained::*};
use semio_framework_job::{OriginalTurnFailure, StepContext};
use semio_framework_value::{
    retained_clone::{RetainedCloneGrant, RetainedCloneStep},
    RetirementDemand, ValueError, ValueRefusalKind,
};

#[derive(Default)]
pub struct RetainedExpressAdmission {
    overlay: PopulationOverlay,
    receiver: PopulationReceiver,
    received: bool,
    closing: bool,
}
impl RetainedExpressAdmission {
    /// 📐 The original caller sees the actual next overlay or semantic receiving frontier.
    pub fn demands(&self, source: &IfcSnapshot, mutations: &[IfcMutation]) -> Result<RetirementDemand, ValueError> {
        if !self.overlay.is_ready() {
            self.overlay.demands(source, mutations)
        } else if !self.received {
            self.receiver.demands(&self.overlay)
        } else {
            Ok(fixed(1))
        }
    }
    /// 🧵 Keep receiving attached to the original captured source and immutable final mutation batch.
    pub fn advance(&mut self, source: &IfcSnapshot, mutations: &[IfcMutation], cx: &mut StepContext<'_>) -> Result<(), OriginalTurnFailure> {
        if self.closing {
            return Err(refusal("express.admission-closing").into());
        }
        if !admitted(cx, self.demands(source, mutations)?)? {
            return Ok(());
        }
        if !self.overlay.is_ready() {
            self.overlay.advance(source, mutations, cx)?;
        } else if !self.received {
            self.received = self.receiver.advance(&self.overlay, source, mutations, cx)?;
        } else {
            return Err(ValueError::literal(
                ValueRefusalKind::UnsupportedOwner,
                "template.semantic-admission-unavailable: retained source receiving is complete, but expression, statement and population rule execution have no retained authority yet",
            )
            .into());
        }
        Ok(())
    }
    pub fn begin_close(&mut self) {
        self.closing = true;
        self.receiver.begin_close();
        self.overlay.begin_close();
    }
    pub fn close_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if !self.receiver.terminal_is_empty() {
            self.receiver.close_demands(body)
        } else {
            self.overlay.close_demands(body)
        }
    }
    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if !self.closing {
            return Err(refusal("express.admission-not-closing"));
        }
        if !self.receiver.terminal_is_empty() {
            self.receiver.close_step(grant)
        } else {
            self.overlay.close_step(grant)
        }
    }
    pub fn terminal_is_empty(&self) -> bool {
        self.receiver.terminal_is_empty() && self.overlay.terminal_is_empty()
    }
}
