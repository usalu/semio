//! 📨️ Diagnostic fragments borrow original optional-text intent owners verbatim.
use super::{Puzzle2dTextDisposition, Puzzle2dTextIntent, Puzzle2dTextPlan};
use crate::standards::v1::subsets::any::schema::snapshot::lookup::Puzzle2dLookupScope;
use protocol::os_store::{ArtifactMessageFragment, ArtifactMessageSource};
use semio_framework_diagnostic::Severity;
use semio_framework_value::paged::Utf8Text;

pub struct Puzzle2dTextDiagnosticSource<'a, T: Puzzle2dTextIntent> {
    payload: &'a T,
    disposition: Puzzle2dTextDisposition,
    operation_index: Option<u32>,
}

impl<'a, T: Puzzle2dTextIntent> Puzzle2dTextDiagnosticSource<'a, T> {
    pub fn new(payload: &'a T, plan: Puzzle2dTextPlan, operation_index: Option<u32>) -> Option<Self> {
        (plan.disposition != Puzzle2dTextDisposition::Changed).then_some(Self { payload, disposition: plan.disposition, operation_index })
    }
}

impl<T: Puzzle2dTextIntent> ArtifactMessageSource for Puzzle2dTextDiagnosticSource<'_, T> {
    fn level(&self) -> Severity { if self.disposition == Puzzle2dTextDisposition::NoOp { Severity::Warning } else { Severity::Error } }
    fn code(&self) -> &'static str { if self.disposition == Puzzle2dTextDisposition::NoOp { "mutation.no-op" } else { "mutation.target-missing" } }
    fn operation_index(&self) -> Option<u32> { self.operation_index }
    fn fragment_count(&self) -> usize { if self.disposition == Puzzle2dTextDisposition::NoOp { 1 } else { 3 } }
    fn fragment(&self, index: usize) -> Option<ArtifactMessageFragment<'_>> {
        match (self.disposition, index) {
            (Puzzle2dTextDisposition::NoOp, 0) => Some(ArtifactMessageFragment::Static("no changes to apply")),
            (Puzzle2dTextDisposition::TargetMissing, 0) => Some(ArtifactMessageFragment::Static(match T::SCOPE { Puzzle2dLookupScope::Region => "target region \"", _ => "node \"" })),
            (Puzzle2dTextDisposition::TargetMissing, 1) => Some(ArtifactMessageFragment::Text(self.payload.identifier())),
            (Puzzle2dTextDisposition::TargetMissing, 2) => Some(ArtifactMessageFragment::Static("\" not found")),
            _ => None,
        }
    }
    fn target_count(&self) -> usize { 1 }
    fn target(&self, index: usize) -> Option<&dyn Utf8Text> { (index == 0).then_some(self.payload.identifier() as &dyn Utf8Text) }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
