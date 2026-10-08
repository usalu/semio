//! 🧬️ `XlsxTransitionalMutation` — the ISO/IEC 29500-4 Transitional CONFORMANCE-CLASS vocabulary of
//! `stdio.xlsx`. Every variant's `diff()` is handcrafted (never apply-and-capture) and every
//! variant's `inverse()` is handcrafted, reading whatever pre-state it needs out of the base. Every diff is built declaratively from the payload and
//! reads of `base`.
//!
//! **Why this subset needs a vocabulary of its own.** `🧱️base` owns the DOCUMENT vocabulary —
//! `insert-sheet`, `remove-sheet`, `rename-sheet`, `set-cell`, `remove-cell` and the three shared-string kinds. Not one of those mutations can move a
//! package between conformance classes, because a conformance class is a property of the OPC
//! PACKAGE and of no document object at all. `check_transitional_conformance` reads four axes: `xl/workbook.xml`'s root `xmlns`, its root `xmlns:r`, a root `conformance` attribute that must NOT say `strict`, and each worksheet part's content type. It has no VML rule at all, because ISO/IEC 29500-4 Transitional deliberately retains VML — so this enum declares two variants fewer than its `🔒️strict` sibling, and that difference is the specification's, not an editorial one.
//!
//! The two vocabularies are disjoint by construction: no `🧱️base` mutation moves an axis this enum
//! addresses, and no variant here touches document content.
//!
//! `Diff` is `XlsxDiff`, the SAME diff type `🧱️base` uses — the two subsets share one snapshot type,
//! so they share its diff. What differs is the vocabulary that produces it, which is what a subset
//! is. `ArtifactBuilder::Mutation` on this subset's builder still names `🧱️base`'s document
//! vocabulary: a builder has exactly one associated mutation type, and a Strict package still needs
//! its content edited. Unifying the two behind one type is a deliberate open seam, recorded rather
//! than guessed at.
//!
//! @see ../../🔣️oracle.json — the mutation catalog `KINDS` is measured against.
//! @see ../🦀️.rs — this subset's conformance check, one axis per variant below.

use crate::standards::v_ecma_376::subsets::base::schema::diff::XlsxDiff;
use crate::standards::v_ecma_376::subsets::base::schema::mutations::{retarget_attribute_values_diff, retype_xml_part_diff, root_attribute_diff, root_edits_diff};
use crate::standards::v_ecma_376::subsets::base::schema::snapshot::XlsxSnapshot;
use protocol::Mutation;
use semio_s_artifact_stdio_xml::schema::snapshot::{XmlDocument, XmlNode};
use semio_s_artifact_stdio_zip::opc::resolve_relationship_target;
use semio_s_artifact_stdio_zip::opc::diff::{OpcDiff, OpcOwnerModification, OpcOwnerPatch, OpcOwnersDelta, OpcRelationshipModification, OpcRelationshipPatch, OpcRelationshipsDelta};

//#region 🔖️Dialect
/// 🏷️ ISO/IEC 29500-4 Transitional SpreadsheetML main namespace.
pub const TRANSITIONAL_MAIN_NS: &str = "http://schemas.openxmlformats.org/spreadsheetml/2006/main";
/// 🏷️ ISO/IEC 29500-1 Strict SpreadsheetML main namespace.
pub const STRICT_MAIN_NS: &str = "http://purl.oclc.org/ooxml/spreadsheetml/main";
/// 🏷️ The main-markup pair, `[transitional, strict]` — the order that makes the class stamp
/// bijective and therefore exactly invertible.
pub const MAIN_NAMESPACES: [&str; 2] = [TRANSITIONAL_MAIN_NS, STRICT_MAIN_NS];

/// 🔗️ ISO/IEC 29500-4 Transitional `officeDocument` relationships namespace and relationship base.
pub const TRANSITIONAL_REL: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";
/// 🔗️ ISO/IEC 29500-1 Strict `officeDocument` relationships namespace and relationship base.
pub const STRICT_REL: &str = "http://purl.oclc.org/ooxml/officeDocument/relationships";
/// 🔗️ The `officeDocument` relationships pair, `[transitional, strict]`.
pub const RELATIONSHIP_NAMESPACES: [&str; 2] = [TRANSITIONAL_REL, STRICT_REL];
//#endregion 🔖️Dialect

//#region 🔖️Mutations
#[path = "🚫️remove-conformance-attribute/🦀️.rs"]
pub mod remove_conformance_attribute;
#[path = "✅️set-conformance-attribute/🦀️.rs"]
pub mod set_conformance_attribute;
#[path = "🌐️set-main-namespace/🦀️.rs"]
pub mod set_main_namespace;
#[path = "🔗️set-relationships-namespace/🦀️.rs"]
pub mod set_relationships_namespace;
/// 📐️ Typed conformance-class mutation for `stdio.xlsx` under ISO/IEC 29500-4
/// Transitional. Every variant addresses ONE axis of the class; none addresses document content.
//#region 🔖️Leaves
#[path = "🔗️set-relationship-base/🦀️.rs"]
pub mod set_relationship_base;
#[path = "🏷️set-worksheet-content-type/🦀️.rs"]
pub mod set_worksheet_content_type;
//#endregion 🔖️Leaves

/// 📐️ Typed mutation for this subset. `NoMutation` was dropped: `#[derive(dsl::Mutations)]` requires
/// every variant to wrap exactly one leaf payload and a unit variant wraps none.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[mutations(snapshot = XlsxSnapshot, diff = XlsxDiff, schema = "XlsxTransitionalMutation")]
pub enum XlsxTransitionalMutation {
    SetMainNamespace(set_main_namespace::SetMainNamespace),
    SetRelationshipsNamespace(set_relationships_namespace::SetRelationshipsNamespace),
    SetRelationshipBase(set_relationship_base::SetRelationshipBase),
    SetConformanceAttribute(set_conformance_attribute::SetConformanceAttribute),
    RemoveConformanceAttribute(remove_conformance_attribute::RemoveConformanceAttribute),
    SetWorksheetContentType(set_worksheet_content_type::SetWorksheetContentType),
}

/// 🧾️ Kebab-case spelling of every `XlsxTransitionalMutation` variant, in declaration order — the exhaustive
/// mutation catalog `xlsx-ecma-376-transitional` (`../../🔣️oracle.json`) is measured against
/// this exact list. `kinds_match_enum_and_catalog` proves it never drifts from either side.
pub const KINDS: &[&str] = &["set-main-namespace", "set-relationships-namespace", "set-relationship-base", "set-conformance-attribute", "remove-conformance-attribute", "set-worksheet-content-type"];
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot` through its own diff — the diff is the single semantics
/// source, never a separate imperative apply path.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
#[cfg(test)]
pub fn apply_xlsx_transitional_mutation(snapshot: &mut XlsxSnapshot, mutation: &XlsxTransitionalMutation) -> protocol::MutationOutcome<XlsxDiff> {
    let outcome = Mutation::diff(mutation, snapshot);
    match protocol::apply_diff(outcome.diff(), snapshot) {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::fatal(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}
/// 🏅️ The concrete mutations that move a package into (`strict`) or out of the strict conformance class: both namespace families, the `officeDocument`
/// relationship base and the main part's `conformance` attribute, each through its own kind.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn stamp_conformance_class_mutations(strict: bool) -> Vec<XlsxTransitionalMutation> {
    let index = usize::from(strict);
    vec![
        XlsxTransitionalMutation::SetMainNamespace(set_main_namespace::SetMainNamespace { namespace: MAIN_NAMESPACES[index].to_string() }),
        XlsxTransitionalMutation::SetRelationshipsNamespace(set_relationships_namespace::SetRelationshipsNamespace { namespace: RELATIONSHIP_NAMESPACES[index].to_string() }),
        XlsxTransitionalMutation::SetRelationshipBase(set_relationship_base::SetRelationshipBase { base: RELATIONSHIP_NAMESPACES[index].to_string() }),
        if strict {
            XlsxTransitionalMutation::SetConformanceAttribute(set_conformance_attribute::SetConformanceAttribute { value: "strict".to_string() })
        } else {
            XlsxTransitionalMutation::RemoveConformanceAttribute(remove_conformance_attribute::RemoveConformanceAttribute {})
        },
    ]
}
//#endregion 🔖️Apply

//#region 🔖️Helpers
/// 🧭️ The main part's path, resolved through the root `officeDocument` relationship by type SUFFIX
/// so it resolves under either conformance class — matching by the transitional-shaped constant
/// verbatim would silently fail to find the main part of a genuinely Strict package.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn main_part_path(base: &XlsxSnapshot) -> Option<String> {
    let relationship = base.opc.relationships_for("").iter().find(|relationship| relationship.rel_type.ends_with("/officeDocument"))?;
    Some(resolve_relationship_target("", &relationship.target))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn declares_namespace(node: &XmlNode, value: &str) -> bool {
    let XmlNode::Element { attrs, children, .. } = node else { return false };
    attrs.iter().any(|attr| attr.value == value) || children.iter().any(|child| declares_namespace(child, value))
}

/// 🔎️ Which member of a `[transitional, strict]` pair the package's logical XML parts actually declare.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn declared_pair_member(base: &XlsxSnapshot, pair: [&str; 2]) -> Option<String> {
    pair.into_iter().find(|candidate| base.xml_parts.iter().any(|part| part.document.root.as_ref().is_some_and(|root| declares_namespace(root, candidate)))).map(str::to_string)
}

/// 🔎️ The relationship-type base the package's own relationships are built on.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn declared_relationship_base(base: &XlsxSnapshot, pair: [&str; 2]) -> Option<String> {
    pair.into_iter().find(|candidate| base.opc.relationships.groups().map(|(_, relationships)| relationships).flatten().any(|relationship| relationship.rel_type.starts_with(candidate))).map(str::to_string)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn root_attribute(document: &XmlDocument, name: &str) -> Option<String> {
    let XmlNode::Element { attrs, .. } = document.root.as_ref()? else { return None };
    attrs.iter().find(|attr| attr.name == name).map(|attr| attr.value.clone())
}

/// 🔎️ The main part's root `conformance` attribute, if it declares one.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn conformance_attribute(base: &XlsxSnapshot) -> Option<String> {
    root_attribute(&base.xml_part(&main_part_path(base)?)?.document, "conformance")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn resolved_content_type(base: &XlsxSnapshot, path: &str) -> Option<String> {
    base.opc.content_types.resolve(path).map(str::to_string)
}
//#endregion 🔖️Helpers

//#region 🔖️DiffBuilders
/// 🔺️ The diff of retargeting one namespace family across every logical XML part that declares it.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_retarget_namespace(base: &XlsxSnapshot, from: [&str; 2], to: &str) -> XlsxDiff {
    root_edits_diff(base.xml_parts.iter().filter_map(|part| retarget_attribute_values_diff(part.document.root.as_ref()?, &from, to).map(|edit| (part.path.clone(), edit))).collect())
}

/// 🔺️ The diff of retargeting the `officeDocument` relationship TYPE base, owner by owner.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_retarget_relationship_base(base: &XlsxSnapshot, from: [&str; 2], to: &str) -> XlsxDiff {
    let mut modified = Vec::new();
    for (owner, relationships) in base.opc.relationships.groups() {
        let entries: Vec<OpcRelationshipModification> = relationships
            .iter()
            .filter_map(|relationship| {
                let prefix = from.into_iter().find(|prefix| relationship.rel_type.starts_with(prefix))?;
                let retargeted = format!("{to}{}", &relationship.rel_type[prefix.len()..]);
                (retargeted != relationship.rel_type).then(|| OpcRelationshipModification { id: relationship.id.clone(), patch: OpcRelationshipPatch { rel_type: Some(retargeted), target: None, target_mode: None } })
            })
            .collect();
        if !entries.is_empty() {
            modified.push(OpcOwnerModification { id: owner.clone(), patch: OpcOwnerPatch { relationships: OpcRelationshipsDelta { modified: entries, ..Default::default() } } });
        }
    }
    if modified.is_empty() {
        return XlsxDiff::default();
    }
    XlsxDiff { opc: Some(OpcDiff { relationships: Some(OpcOwnersDelta { modified, ..Default::default() }), ..Default::default() }), xml_parts: None }
}

/// 🔺️ The diff of setting — or removing — the main part's root `conformance` attribute.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_conformance_attribute(base: &XlsxSnapshot, value: Option<&str>) -> XlsxDiff {
    let Some(path) = main_part_path(base) else { return XlsxDiff::default() };
    let Some(part) = base.xml_part(&path) else { return XlsxDiff::default() };
    root_attribute_diff(&part.document, "conformance", value).map(|edit| root_edits_diff(vec![(part.path.clone(), edit)])).unwrap_or_default()
}

/// 🔺️ The diff of retyping one part: the logical XML part's own `content_type` and its `[Content_Types].xml` override move together, because the two can
/// never be allowed to drift apart.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_set_content_type(base: &XlsxSnapshot, path: &str, content_type: &str, override_index: Option<usize>) -> XlsxDiff {
    retype_xml_part_diff(base, path, content_type, override_index)
}
//#endregion 🔖️DiffBuilders

//#region 🔖️Inverses
/// ↩️ The mutation that restores the main namespace the package declares.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn namespace_inverse(base: &XlsxSnapshot) -> Vec<XlsxTransitionalMutation> {
    declared_pair_member(base, MAIN_NAMESPACES).map(|namespace| XlsxTransitionalMutation::SetMainNamespace(set_main_namespace::SetMainNamespace { namespace })).into_iter().collect()
}

/// ↩️ The mutation that restores the relationships namespace the package declares.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn relationships_namespace_inverse(base: &XlsxSnapshot) -> Vec<XlsxTransitionalMutation> {
    declared_pair_member(base, RELATIONSHIP_NAMESPACES).map(|namespace| XlsxTransitionalMutation::SetRelationshipsNamespace(set_relationships_namespace::SetRelationshipsNamespace { namespace })).into_iter().collect()
}

/// ↩️ The mutation that restores the `officeDocument` relationship base the package declares.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn relationship_base_inverse(base: &XlsxSnapshot) -> Vec<XlsxTransitionalMutation> {
    declared_relationship_base(base, RELATIONSHIP_NAMESPACES).map(|target| XlsxTransitionalMutation::SetRelationshipBase(set_relationship_base::SetRelationshipBase { base: target })).into_iter().collect()
}

/// ↩️ The mutation that restores the main part's `conformance` attribute: its value, or its absence.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn conformance_attribute_inverse(base: &XlsxSnapshot, forward_changes: bool) -> Vec<XlsxTransitionalMutation> {
    match conformance_attribute(base) {
        Some(value) => vec![XlsxTransitionalMutation::SetConformanceAttribute(set_conformance_attribute::SetConformanceAttribute { value })],
        None if forward_changes => vec![XlsxTransitionalMutation::RemoveConformanceAttribute(remove_conformance_attribute::RemoveConformanceAttribute {})],
        None => Vec::new(),
    }
}

/// ↩️ The mutation that restores the content type the part resolved to, with its explicit override at the position it held.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn worksheet_content_type_inverse(base: &XlsxSnapshot, path: &str) -> Vec<XlsxTransitionalMutation> {
    let name = format!("/{}", path.trim_start_matches('/'));
    let override_index = base.opc.content_types.overrides.iter().position(|(existing, _)| *existing == name);
    resolved_content_type(base, path).map(|content_type| XlsxTransitionalMutation::SetWorksheetContentType(set_worksheet_content_type::SetWorksheetContentType { path: path.to_string(), content_type, override_index })).into_iter().collect()
}
//#endregion 🔖️Inverses

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
