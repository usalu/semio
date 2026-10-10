//! 🧬️ Logical DWG document mutations.

use crate::schema::diff::{self, DwgDiff};
use crate::DwgSnapshot;
use protocol::Mutation;

//#region 🔖️Mutations
//#region 🔖️Leaves
#[path = "🏷️set-version-info/🦀️.rs"]
pub mod set_version_info;
#[path = "📐️set-drawing/🦀️.rs"]
pub mod set_drawing;
#[path = "🧾️set-header/🦀️.rs"]
pub mod set_header;
#[path = "🗂️set-classes/🦀️.rs"]
pub mod set_classes;
#[path = "🔗️set-dependencies/🦀️.rs"]
pub mod set_dependencies;
#[path = "📝️set-summary/🦀️.rs"]
pub mod set_summary;
#[path = "🛠️set-application/🦀️.rs"]
pub mod set_application;
#[path = "📄️set-template/🦀️.rs"]
pub mod set_template;
#[path = "🧰️set-auxiliary-header/🦀️.rs"]
pub mod set_auxiliary_header;
#[path = "🕘️set-revision-history/🦀️.rs"]
pub mod set_revision_history;
#[path = "🖼️set-preview/🦀️.rs"]
pub mod set_preview;
#[path = "📜️set-application-history/🦀️.rs"]
pub mod set_application_history;
//#endregion 🔖️Leaves

/// 📐️ Typed mutation for this subset. `NoMutation` was dropped: `#[derive(dsl::Mutations)]` requires
/// every variant to wrap exactly one leaf payload and a unit variant wraps none, and `no` is not an
/// approved semantic verb.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations, semio_framework_value::RetireOwned)]
#[mutations(snapshot = DwgSnapshot, diff = DwgDiff, schema = "DwgMutation")]
#[value(tag = "mutation", rename_all = "camelCase")]
pub enum DwgMutation {
    SetVersionInfo(set_version_info::SetVersionInfo),
    SetDrawing(set_drawing::SetDrawing),
    SetHeader(set_header::SetHeader),
    SetClasses(set_classes::SetClasses),
    SetDependencies(set_dependencies::SetDependencies),
    SetSummary(set_summary::SetSummary),
    SetApplication(set_application::SetApplication),
    SetTemplate(set_template::SetTemplate),
    SetAuxiliaryHeader(set_auxiliary_header::SetAuxiliaryHeader),
    SetRevisionHistory(set_revision_history::SetRevisionHistory),
    SetPreview(set_preview::SetPreview),
    SetApplicationHistory(set_application_history::SetApplicationHistory),
}



//#endregion 🔖️Mutations

//#region 🔖️Codecs

//#endregion 🔖️Codecs

//#region 🔖️Kinds
impl DwgMutation {
    /// 🏷️ Kebab-case kind spelling — the exact vocabulary BOTH DWG catalogs declare
    /// (`../../🔣️oracle.json` and `../../../../4️⃣ac1018/🪆️subsets/✳️any/🔮️oracles/
    /// 🔣️.json`), and the row ids of both cases' Scenario Outlines. Hand-matched rather
    /// than derived, so [`KINDS`] is checked against something with its own reason to be right; and
    /// exhaustive, so a variant added to the enum is a COMPILE error here rather than a silently
    /// uncatalogued kind.
    // 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
    pub fn kind(&self) -> &'static str {
        match self {
            DwgMutation::SetVersionInfo(_) => "set-version-info",
            DwgMutation::SetDrawing(_) => "set-drawing",
            DwgMutation::SetHeader(_) => "set-header",
            DwgMutation::SetClasses(_) => "set-classes",
            DwgMutation::SetDependencies(_) => "set-dependencies",
            DwgMutation::SetSummary(_) => "set-summary",
            DwgMutation::SetApplication(_) => "set-application",
            DwgMutation::SetTemplate(_) => "set-template",
            DwgMutation::SetAuxiliaryHeader(_) => "set-auxiliary-header",
            DwgMutation::SetRevisionHistory(_) => "set-revision-history",
            DwgMutation::SetPreview(_) => "set-preview",
            DwgMutation::SetApplicationHistory(_) => "set-application-history",
        }
    }
}

/// 🏷️ Every declared kind, kebab-case, in the enum's own declaration order. ⚠️ It mirrors TWO
/// catalogs, not one: `4️⃣ac1018/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs` is a `pub use`
/// of this module, so AC1018 declares this same vocabulary and both manifests must list it.
pub const KINDS: &[&str] = &["set-version-info", "set-drawing", "set-header", "set-classes", "set-dependencies", "set-summary", "set-application", "set-template", "set-auxiliary-header", "set-revision-history", "set-preview", "set-application-history"];
//#endregion 🔖️Kinds


//#endregion 🔖️MutationTrait

#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_mutation_cases() -> Vec<DwgMutation> {
    use crate::schema::snapshot::*;
    vec![
        DwgMutation::SetVersionInfo(set_version_info::SetVersionInfo { version: "AC1024".into(), maintenance_version: 9, codepage: 65001 }),
        DwgMutation::SetDrawing(set_drawing::SetDrawing { drawing: DwgLogicalDrawing::default() }),
        DwgMutation::SetHeader(set_header::SetHeader { header: DwgHeaderVariables::default() }),
        DwgMutation::SetClasses(set_classes::SetClasses { classes: vec![DwgClass { number: 500, dxf_name: "ACDBPLACEHOLDER".into(), ..Default::default() }] }),
        DwgMutation::SetDependencies(set_dependencies::SetDependencies { dependencies: vec![DwgDependency { feature: "xref".into(), relative_path: "site.dwg".into(), ..Default::default() }] }),
        DwgMutation::SetSummary(set_summary::SetSummary { summary: DwgSummaryInfo { title: "Architectural".into(), ..Default::default() } }),
        DwgMutation::SetApplication(set_application::SetApplication { application: DwgApplicationInfo { product: "AutoCAD".into(), ..Default::default() } }),
        DwgMutation::SetTemplate(set_template::SetTemplate { template: DwgTemplate::default() }),
        DwgMutation::SetAuxiliaryHeader(set_auxiliary_header::SetAuxiliaryHeader { auxiliary_header: DwgAuxiliaryHeader::default() }),
        DwgMutation::SetRevisionHistory(set_revision_history::SetRevisionHistory { revision_history: DwgRevisionHistory::default() }),
        DwgMutation::SetPreview(set_preview::SetPreview { preview: DwgIndexedPreview::default() }),
        DwgMutation::SetApplicationHistory(set_application_history::SetApplicationHistory { application_history: DwgApplicationHistory::default() }),
    ]
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

//#region 🧪️FixtureCases
//#endregion 🧪️FixtureCases
