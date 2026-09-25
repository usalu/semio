//! 🧩️ Authoritative PDF mutation payload, diff, inverse, and tests for `set-pattern`.

use super::PdfMutation;
use crate::standards::v1_7::subsets::base::schema::{
    diff::{self, PdfDiff},
    snapshot::*,
};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetPattern {
    pub pattern: PdfPattern,
}

impl MutationKind<PdfSnapshot, PdfMutation> for SetPattern {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "pattern", kind: "set-pattern", record: "Set" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        match unresolved_reference(&self.pattern, base) {
            Some((resource, id)) => MutationOutcome::error("mutation.target-missing", format!("Pattern \"{}\" names the {resource} \"{id}\", which the document does not hold.", self.pattern.id), [self.pattern.id.clone()]),
            None => MutationOutcome::new(diff::diff_set_pattern(base, self.pattern.clone())),
        }
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfMutation> {
        let _ = base;
        match base.patterns.iter().find(|item| item.id == self.pattern.id) { Some(previous) => vec![PdfMutation::SetPattern(SetPattern { pattern: previous.clone() })], None => vec![PdfMutation::RemovePattern(super::remove_pattern::RemovePattern { id: self.pattern.id.clone() })] }
    }

    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Set pattern {}", self.pattern.id), &format!("Muster {} setzen", self.pattern.id))
    }

    fn target(&self) -> Vec<String> {
        vec![self.pattern.id.clone()]
    }
}

/// 🧷️ The first resource a shading pattern names that `base` does not hold — its `/Shading`, then its
/// `/ExtGState` (ISO 32000-1 §8.7.4.3). The writer can only emit a reference to a resource of the document,
/// so accepting a dangling one would write a pattern that re-reads differently. An empty shading id is the
/// reader's spelling of an absent `/Shading` and round-trips unchanged, so it names nothing.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via the synchronous `MutationKind::diff`) — see R9
fn unresolved_reference<'a>(pattern: &'a PdfPattern, base: &PdfSnapshot) -> Option<(&'static str, &'a str)> {
    let PdfPatternKind::Shading { shading, ext_g_state } = &pattern.kind else { return None };
    let shading = (!shading.is_empty() && !base.shadings.iter().any(|item| item.id == *shading)).then_some(("shading", shading.as_str()));
    shading.or_else(|| ext_g_state.as_deref().filter(|id| !base.ext_g_states.iter().any(|item| item.id == *id)).map(|id| ("extended graphics state", id)))
}
//#endregion 🔖️Mutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[path = "💾️binary/🦀️.rs"]
pub mod binary;
#[path = "📝️text/🦀️.rs"]
pub mod text;
