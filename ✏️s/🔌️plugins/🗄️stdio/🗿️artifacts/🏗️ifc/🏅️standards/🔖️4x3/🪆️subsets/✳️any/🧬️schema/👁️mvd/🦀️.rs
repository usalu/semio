//! 👁️ Published ADD2 entity and concept scope, separate from exchange conformance.

mod header;
#[path = "🧾️requirements/coverage.rs"]
mod requirements_coverage;
pub use header::{IfcMvdDeclaration, ifc4x3_mvd_declarations, ifc4x3_mvd_header_mutation, ifc4x3_mvd_header_text};
#[path = "🧩️concepts/native.rs"]
mod native;
#[path = "🧩️concepts/types.rs"]
mod concept_types;
pub use concept_types::{IfcConceptDiagnostic, IfcConceptLabel, IfcConceptSource, IfcConceptTarget, IfcConceptTargetReport};
#[path = "../../../../../../📥️source/🗂️index/🦀️.rs"]
mod source_index;
#[path = "🧩️concepts/preparation.rs"]
mod preparation;
#[path = "🧩️concepts/population.rs"]
mod population;
#[path = "🧩️concepts/🧱️body/🦀️.rs"]
mod body;
#[path = "🧩️concepts/📐️alignment/roles.rs"]
mod alignment_roles;

pub use body::{IfcBodyConceptRequest, IfcBodyConceptReport, IfcBodyConceptWork};
#[path = "🧩️concepts/📐️alignment/direct.rs"]
mod alignment_direct;
pub use alignment_direct::{IfcAlignmentDirectTarget,IfcAlignmentDirectRequest,IfcAlignmentDirectTargetReport,IfcAlignmentDirectReport,IfcAlignmentDirectWork};
#[path="🧩️concepts/📐️alignment/terminal.rs"]
mod alignment_terminal;
pub use alignment_terminal::{IfcAlignmentTerminalTarget,IfcAlignmentTerminalRequest,IfcAlignmentTerminalTargetReport,IfcAlignmentTerminalReport,IfcAlignmentTerminalWork};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IfcMvdView {
    Reference,
    Alignment,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IfcMvdLocale {
    En,
    De,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IfcMvdScopeState {
    Universal,
    Included,
    Unlisted,
    Unknown,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IfcMvdScope {
    pub state: IfcMvdScopeState,
    pub causes: &'static [&'static str],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IfcMvdEntry {
    pub id: &'static str,
    pub name: &'static str,
    pub href: &'static str,
    pub universal: bool,
    pub reference: Option<&'static [&'static str]>,
    pub alignment: Option<&'static [&'static str]>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IfcMvdSource {
    pub url: &'static str,
    pub sha256: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IfcMvdCatalog {
    pub schema: &'static str,
    pub source_commit: &'static str,
    pub sources: &'static [IfcMvdSource],
    pub conformance_evaluated: bool,
    pub entities: &'static [IfcMvdEntry],
    pub concepts: &'static [IfcMvdEntry],
}

include!("data.rs");

/// 📚️ Read immutable official scope data without a runtime parser or third-party dependency.
pub fn ifc4x3_mvd_catalog() -> &'static IfcMvdCatalog {
    &CATALOG
}

fn scope(entry: Option<&'static IfcMvdEntry>, view: IfcMvdView) -> IfcMvdScope {
    let Some(entry) = entry else { return IfcMvdScope { state: IfcMvdScopeState::Unknown, causes: &[] } };
    let causes = match view {
        IfcMvdView::Reference => entry.reference,
        IfcMvdView::Alignment => entry.alignment,
    };
    IfcMvdScope {
        state: if entry.universal {
            IfcMvdScopeState::Universal
        } else if causes.is_some() {
            IfcMvdScopeState::Included
        } else {
            IfcMvdScopeState::Unlisted
        },
        causes: causes.unwrap_or(&[]),
    }
}

/// 🏛️ Inspect exact declared entity scope while preserving unrestricted repair access.
pub fn ifc4x3_mvd_entity_scope(entity: &str, view: IfcMvdView) -> IfcMvdScope {
    let id = entity.to_ascii_uppercase();
    scope(CATALOG.entities.binary_search_by_key(&id.as_str(), |entry| entry.id).ok().map(|index| &CATALOG.entities[index]), view)
}

/// 🧩 Inspect a concept by its exact documentation path.
pub fn ifc4x3_mvd_concept_scope(id: &str, view: IfcMvdView) -> IfcMvdScope {
    scope(CATALOG.concepts.binary_search_by_key(&id, |entry| entry.id).ok().map(|index| &CATALOG.concepts[index]), view)
}

/// 🗣️ Resolve a view's label using an explicit supported language.
pub fn ifc4x3_mvd_label(view: IfcMvdView, locale: IfcMvdLocale) -> &'static str {
    match (view, locale) {
        (IfcMvdView::Reference, IfcMvdLocale::En) => "Reference View",
        (IfcMvdView::Reference, IfcMvdLocale::De) => "Referenzansicht",
        (IfcMvdView::Alignment, IfcMvdLocale::En) => "Alignment Based View",
        (IfcMvdView::Alignment, IfcMvdLocale::De) => "Trassierungsbasierte Ansicht",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neutral_scope_cases_match_the_public_native_interface() {
        let cases: serde_json::Value = serde_json::from_str(include_str!("../../🧪️tests/mvd-cases.json")).unwrap();
        let view = |id: &str| match id {
            "ReferenceView" => IfcMvdView::Reference,
            "Alignment-basedReferenceView" => IfcMvdView::Alignment,
            _ => panic!("fixture view must be published"),
        };
        let state = |value| match value {
            IfcMvdScopeState::Universal => "universal",
            IfcMvdScopeState::Included => "included",
            IfcMvdScopeState::Unlisted => "unlisted",
            IfcMvdScopeState::Unknown => "unknown",
        };
        for row in cases["cases"].as_array().unwrap() {
            assert_eq!(state(ifc4x3_mvd_entity_scope(row["entity"].as_str().unwrap(), view(row["view"].as_str().unwrap())).state), row["state"].as_str().unwrap());
        }
        for row in cases["concepts"].as_array().unwrap() {
            assert_eq!(state(ifc4x3_mvd_concept_scope(row["id"].as_str().unwrap(), view(row["view"].as_str().unwrap())).state), row["state"].as_str().unwrap());
        }
        assert_eq!(CATALOG.entities.len(), crate::catalog::ifc4x3_catalog().entities.len());
        assert!(!CATALOG.conformance_evaluated);
        assert_eq!(ifc4x3_mvd_label(IfcMvdView::Reference, IfcMvdLocale::De), "Referenzansicht");
    }
}
