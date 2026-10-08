//! 🧬️ `DocxStrictMutation` — the ISO/IEC 29500-1 Strict CONFORMANCE-CLASS vocabulary of
//! `stdio.docx`. Every variant's `diff()` is handcrafted (never apply-and-capture) and every
//! variant's `inverse()` is handcrafted, reading whatever pre-state it needs out of the base.
//!
//! **Why this subset needs a vocabulary of its own.** `✳️any` owns the DOCUMENT vocabulary —
//! `insert-block`, `remove-block`, `set-block-content`, `set-run-text`, the style kinds and the part kinds. Not one of those mutations can move a
//! package between conformance classes, because a conformance class is a property of the OPC
//! PACKAGE and of no document object at all. `check_strict_conformance` reads six axes on an already-decoded `DocxSnapshot`: the main document part's Strict WordprocessingML namespace, the Transitional namespace anywhere in the package, the VML namespace anywhere in the package, the `officeDocument` relationship base of every relationship, the main part's root `conformance="strict"`, and `mc:AlternateContent` compatibility markup. This enum is one variant per axis, plus the two baseline variants.
//!
//! The two vocabularies are disjoint by construction: no `✳️any` mutation moves an axis this enum
//! addresses, and no variant here touches document content.
//!
//! `Diff` is `DocxDiff`, the SAME diff type `✳️any` uses — the two subsets share one snapshot type,
//! so they share its diff. What differs is the vocabulary that produces it, which is what a subset
//! is. `ArtifactBuilder::Mutation` on this subset's builder still names `✳️any`'s document
//! vocabulary: a builder has exactly one associated mutation type, and a Strict package still needs
//! its content edited. Unifying the two behind one type is a deliberate open seam, recorded rather
//! than guessed at.
//!
//! @see ../../🔣️oracle.json — the mutation catalog `KINDS` is measured against.
//! @see ../🦀️.rs — this subset's conformance check, one axis per variant below.

use crate::standards::v_ecma_376::subsets::base::schema::diff::DocxDiff;
use crate::standards::v_ecma_376::subsets::base::schema::snapshot::{DocxSnapshot, DocxXmlPart};
use protocol::Mutation;
use semio_s_artifact_stdio_xml::schema::snapshot::{XmlAttr, XmlDocument, XmlNode};
use semio_s_artifact_stdio_zip::opc::resolve_relationship_target;
use semio_s_artifact_stdio_zip::opc::diff::{OpcContentTypesDiff, OpcDiff, OpcOwnerModification, OpcOwnerPatch, OpcOwnersDelta, OpcPartsDelta, OpcRelationshipModification, OpcRelationshipPatch, OpcRelationshipsDelta};

//#region 🔖️Dialect
/// 🏷️ ISO/IEC 29500-4 Transitional WordprocessingML main namespace.
pub const TRANSITIONAL_MAIN_NS: &str = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";
/// 🏷️ ISO/IEC 29500-1 Strict WordprocessingML main namespace.
pub const STRICT_MAIN_NS: &str = "http://purl.oclc.org/ooxml/wordprocessingml/main";
/// 🏷️ The main-markup pair, `[transitional, strict]` — the order that makes the class stamp
/// bijective and therefore exactly invertible.
pub const MAIN_NAMESPACES: [&str; 2] = [TRANSITIONAL_MAIN_NS, STRICT_MAIN_NS];

/// 🔗️ ISO/IEC 29500-4 Transitional `officeDocument` relationships namespace and relationship base.
pub const TRANSITIONAL_REL: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";
/// 🔗️ ISO/IEC 29500-1 Strict `officeDocument` relationships namespace and relationship base.
pub const STRICT_REL: &str = "http://purl.oclc.org/ooxml/officeDocument/relationships";
/// 🔗️ The `officeDocument` relationships pair, `[transitional, strict]`.
pub const RELATIONSHIP_NAMESPACES: [&str; 2] = [TRANSITIONAL_REL, STRICT_REL];

/// 🧩️ The legacy VML namespace ISO/IEC 29500-1 Strict removes entirely.
pub const VML_NS: &str = "urn:schemas-microsoft-com:vml";
/// 🧩️ The content type a legacy VML drawing part resolves.
pub const VML_CONTENT_TYPE: &str = "application/vnd.openxmlformats-officedocument.vmlDrawing";

/// 🧩️ The markup-compatibility namespace an `mc:AlternateContent` fallback declares.
pub const MARKUP_COMPATIBILITY_NS: &str = "http://schemas.openxmlformats.org/markup-compatibility/2006";
/// 🧩️ The element name a markup-compatibility fallback carries.
pub const ALTERNATE_CONTENT_ELEMENT: &str = "mc:AlternateContent";
//#endregion 🔖️Dialect

//#region 🔖️Mutations
#[path = "🪆️insert-alternate-content/🦀️.rs"]
pub mod insert_alternate_content;
#[path = "✒️insert-vml-part/🦀️.rs"]
pub mod insert_vml_part;
#[path = "📤️remove-alternate-content/🦀️.rs"]
pub mod remove_alternate_content;
#[path = "🚫️remove-conformance-attribute/🦀️.rs"]
pub mod remove_conformance_attribute;
#[path = "🧹️remove-vml-part/🦀️.rs"]
pub mod remove_vml_part;
#[path = "✅️set-conformance-attribute/🦀️.rs"]
pub mod set_conformance_attribute;
#[path = "🌐️set-main-namespace/🦀️.rs"]
pub mod set_main_namespace;
#[path = "🔗️set-relationship-base/🦀️.rs"]
pub mod set_relationship_base;
/// 📐️ Typed conformance-class mutation for `stdio.docx` under ISO/IEC 29500-1
/// Strict. Every variant addresses ONE axis of the class; none addresses document content.
//#region 🔖️Leaves
//#endregion 🔖️Leaves

/// 📐️ Typed mutation for this subset. `NoMutation` was dropped: `#[derive(dsl::Mutations)]` requires
/// every variant to wrap exactly one leaf payload and a unit variant wraps none.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[mutations(snapshot = DocxSnapshot, diff = DocxDiff, schema = "DocxStrictMutation")]
pub enum DocxStrictMutation {
    SetMainNamespace(set_main_namespace::SetMainNamespace),
    SetRelationshipBase(set_relationship_base::SetRelationshipBase),
    SetConformanceAttribute(set_conformance_attribute::SetConformanceAttribute),
    RemoveConformanceAttribute(remove_conformance_attribute::RemoveConformanceAttribute),
    InsertVmlPart(insert_vml_part::InsertVmlPart),
    RemoveVmlPart(remove_vml_part::RemoveVmlPart),
    InsertAlternateContent(insert_alternate_content::InsertAlternateContent),
    RemoveAlternateContent(remove_alternate_content::RemoveAlternateContent),
}

/// 🧾️ Kebab-case spelling of every `DocxStrictMutation` variant, in declaration order — the exhaustive
/// mutation catalog `docx-ecma-376-strict` (`../../🔣️oracle.json`) is measured against
/// this exact list. `kinds_match_enum_and_catalog` proves it never drifts from either side.
pub const KINDS: &[&str] = &["set-main-namespace", "set-relationship-base", "set-conformance-attribute", "remove-conformance-attribute", "insert-vml-part", "remove-vml-part", "insert-alternate-content", "remove-alternate-content"];
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot` through its own diff — the diff is the single semantics
/// source, never a separate imperative apply path.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
#[cfg(test)]
pub fn apply_docx_strict_mutation(snapshot: &mut DocxSnapshot, mutation: &DocxStrictMutation) -> protocol::MutationOutcome<DocxDiff> {
    let outcome = Mutation::diff(mutation, snapshot);
    match protocol::apply_diff(outcome.diff(), snapshot) {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::fatal(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}

/// 🏅️ The concrete mutations that move a package into (`strict`) or out of the strict conformance class: the main namespace, the
/// `officeDocument` relationship base and the main part's `conformance` attribute, each through its own kind.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn stamp_conformance_class_mutations(strict: bool) -> Vec<DocxStrictMutation> {
    let index = usize::from(strict);
    vec![
        DocxStrictMutation::SetMainNamespace(set_main_namespace::SetMainNamespace { namespace: MAIN_NAMESPACES[index].to_string() }),
        DocxStrictMutation::SetRelationshipBase(set_relationship_base::SetRelationshipBase { base: RELATIONSHIP_NAMESPACES[index].to_string() }),
        if strict {
            DocxStrictMutation::SetConformanceAttribute(set_conformance_attribute::SetConformanceAttribute { value: "strict".to_string() })
        } else {
            DocxStrictMutation::RemoveConformanceAttribute(remove_conformance_attribute::RemoveConformanceAttribute {})
        },
    ]
}
//#endregion 🔖️Apply

//#region 🔖️Helpers
/// 🧭️ The main part's path, resolved through the root `officeDocument` relationship by type SUFFIX
/// so it resolves under either conformance class — matching by the transitional-shaped constant
/// verbatim would silently fail to find the main part of a genuinely Strict package.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn main_part_path(base: &DocxSnapshot) -> Option<String> {
    let relationship = base.opc.relationships_for("")?.iter().find(|relationship| relationship.rel_type.to_string_owner().ends_with("/officeDocument"))?;
    Some(resolve_relationship_target("", &relationship.target.to_string_owner()))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_part(part: &DocxXmlPart) -> Option<XmlDocument> {
    part.materialize_document_exact().ok()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn declares_namespace(node: &XmlNode, value: &str) -> bool {
    let XmlNode::Element { attrs, children, .. } = node else { return false };
    attrs.iter().any(|attr| attr.value == value) || children.iter().any(|child| declares_namespace(child, value))
}

/// 🔎️ Which member of a `[transitional, strict]` pair the package actually declares.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn declared_pair_member(base: &DocxSnapshot, pair: [&str; 2]) -> Option<String> {
    pair.into_iter().find(|candidate| base.xml_parts.iter().filter_map(parse_part).any(|document| document.root.as_ref().is_some_and(|root| declares_namespace(root, candidate)))).map(str::to_string)
}

/// 🔎️ The relationship-type base the package's own relationships are built on.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn declared_relationship_base(base: &DocxSnapshot, pair: [&str; 2]) -> Option<String> {
    pair.into_iter().find(|candidate| base.opc.relationships.values().flat_map(|relationships| relationships.iter()).any(|relationship| relationship.rel_type.to_string_owner().starts_with(candidate))).map(str::to_string)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn root_attribute(document: &XmlDocument, name: &str) -> Option<String> {
    let XmlNode::Element { attrs, .. } = document.root.as_ref()? else { return None };
    attrs.iter().find(|attr| attr.name == name).map(|attr| attr.value.clone())
}

/// 🔎️ The main part's root `conformance` attribute, if it declares one.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn conformance_attribute(base: &DocxSnapshot) -> Option<String> {
    root_attribute(&parse_part(base.xml_part(&main_part_path(base)?)?)?, "conformance")
}

//#endregion 🔖️Helpers

//#region 🔖️DiffBuilders
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn opc_diff(parts: Option<OpcPartsDelta>, content_types: Option<OpcContentTypesDiff>, relationships: Option<OpcOwnersDelta>) -> DocxDiff {
    if parts.is_none() && content_types.is_none() && relationships.is_none() {
        return DocxDiff::default();
    }
    DocxDiff { opc: Some(OpcDiff { content_types, parts, relationships, comment: None }), ..Default::default() }
}

/// 🔺️ The diff of retargeting the `officeDocument` relationship TYPE base, owner by owner.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_retarget_relationship_base(base: &DocxSnapshot, from: [&str; 2], to: &str) -> DocxDiff {
    let mut owners: Vec<_> = base.opc.relationships.keys().collect();
    owners.sort();
    let mut modified = Vec::new();
    for owner in owners {
        let mut entries = Vec::new();
        for relationship in base.opc.relationships.get(owner).expect("enumerated retained relationship owner").iter() {
            let current = relationship.rel_type.to_string_owner();
            let Some(prefix) = from.into_iter().find(|prefix| current.starts_with(prefix)) else { continue };
            let retargeted = format!("{to}{}", &current[prefix.len()..]);
            if retargeted == current {
                continue;
            }
            entries.push(OpcRelationshipModification { id: relationship.id.to_string_owner(), patch: OpcRelationshipPatch { rel_type: Some(retargeted), target: None, target_mode: None } });
        }
        if entries.is_empty() {
            continue;
        }
        modified.push(OpcOwnerModification { id: owner.to_string_owner(), patch: OpcOwnerPatch { relationships: OpcRelationshipsDelta { modified: entries, ..Default::default() } } });
    }
    if modified.is_empty() {
        return DocxDiff::default();
    }
    opc_diff(None, None, Some(OpcOwnersDelta { modified, ..Default::default() }))
}

/// 🔺️ The diff of retargeting one namespace family across every XML part that declares it.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_retarget_namespace(base: &DocxSnapshot, from: [&str; 2], to: &str) -> DocxDiff {
    crate::standards::v_ecma_376::subsets::base::schema::mutations::root_edits_diff(base.xml_parts.iter().filter_map(|part| {
        let document = parse_part(part)?;
        crate::standards::v_ecma_376::subsets::base::schema::mutations::retarget_attribute_values_diff(document.root.as_ref()?, &from, to).map(|edit| (part.path.clone(), edit))
    }).collect())
}

/// 🔺️ The diff of setting — or removing — the main part's root `conformance` attribute.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_conformance_attribute(base: &DocxSnapshot, value: Option<&str>) -> DocxDiff {
    let Some(path) = main_part_path(base) else { return DocxDiff::default() };
    let Some(document) = base.xml_part(&path).and_then(parse_part) else { return DocxDiff::default() };
    crate::standards::v_ecma_376::subsets::base::schema::mutations::root_attribute_diff(&document, "conformance", value).map(|edit| crate::standards::v_ecma_376::subsets::base::schema::mutations::root_edits_diff(vec![(path, edit)])).unwrap_or_default()
}

/// 🔺️ The diff of adding a legacy VML drawing part, at `index` among the XML parts and with its override at `override_index` (each appended when `None`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_insert_vml_part(base: &DocxSnapshot, path: &str, document: &XmlDocument, index: Option<usize>, override_index: Option<usize>) -> DocxDiff {
    crate::standards::v_ecma_376::subsets::base::schema::mutations::insert_xml_part_diff(base, path, VML_CONTENT_TYPE, document, index, override_index).unwrap_or_default()
}

/// 🔺️ The diff of removing a legacy VML drawing part and its content-type override.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_remove_vml_part(base: &DocxSnapshot, path: &str) -> DocxDiff {
    crate::standards::v_ecma_376::subsets::base::schema::mutations::remove_xml_part_diff(base, path)
}

/// 🧩️ The canonical markup-compatibility fallback this vocabulary inserts.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn alternate_content_node() -> XmlNode {
    XmlNode::Element {
        name: ALTERNATE_CONTENT_ELEMENT.to_string(),
        attrs: vec![XmlAttr { name: "xmlns:mc".to_string(), value: MARKUP_COMPATIBILITY_NS.to_string() }],
        children: vec![XmlNode::Element { name: "mc:Fallback".to_string(), attrs: vec![], children: vec![] }],
    }
}


/// 🧭️ The root children of `document` that are markup-compatibility fallbacks, optionally only the one at `only`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn alternate_content_positions(document: &XmlDocument, only: Option<usize>) -> Vec<(usize, XmlNode)> {
    let Some(XmlNode::Element { children, .. }) = document.root.as_ref() else { return Vec::new() };
    children.iter().enumerate().filter(|(index, child)| only.is_none_or(|only| only == *index) && matches!(child, XmlNode::Element { name, .. } if name == ALTERNATE_CONTENT_ELEMENT)).map(|(index, child)| (index, child.clone())).collect()
}

/// 🧭️ The final index a fallback inserted at `index` lands on in the root of `document`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn alternate_content_landing(document: &XmlDocument, index: Option<usize>) -> usize {
    let length = match document.root.as_ref() {
        Some(XmlNode::Element { children, .. }) => children.len(),
        _ => 0,
    };
    index.map_or(length, |index| index.min(length))
}

/// 🔺️ The diff of inserting one markup-compatibility fallback (`node`, canonical by default) into a part's root element at `index` (appended when `None`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_insert_alternate_content(base: &DocxSnapshot, path: &str, node: Option<&XmlNode>, index: Option<usize>) -> DocxDiff {
    let Some(document) = base.xml_part(path).and_then(parse_part) else { return DocxDiff::default() };
    let landing = alternate_content_landing(&document, index);
    crate::standards::v_ecma_376::subsets::base::schema::mutations::root_children_diff(&document, Some((landing, node.cloned().unwrap_or_else(alternate_content_node))), &[]).map(|edit| crate::standards::v_ecma_376::subsets::base::schema::mutations::root_edits_diff(vec![(path.to_string(), edit)])).unwrap_or_default()
}

/// 🔺️ The diff of stripping the markup-compatibility fallbacks from a part's root element: every one, or only the one at `index`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_remove_alternate_content(base: &DocxSnapshot, path: &str, index: Option<usize>) -> DocxDiff {
    let Some(document) = base.xml_part(path).and_then(parse_part) else { return DocxDiff::default() };
    let removed: Vec<usize> = alternate_content_positions(&document, index).into_iter().map(|(position, _)| position).collect();
    crate::standards::v_ecma_376::subsets::base::schema::mutations::root_children_diff(&document, None, &removed).map(|edit| crate::standards::v_ecma_376::subsets::base::schema::mutations::root_edits_diff(vec![(path.to_string(), edit)])).unwrap_or_default()
}
//#endregion 🔖️DiffBuilders

//#region 🔖️Inverses
/// ↩️ The mutation that restores the main namespace the package declares.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn namespace_inverse(base: &DocxSnapshot) -> Vec<DocxStrictMutation> {
    declared_pair_member(base, MAIN_NAMESPACES).map(|namespace| DocxStrictMutation::SetMainNamespace(set_main_namespace::SetMainNamespace { namespace })).into_iter().collect()
}

/// ↩️ The mutation that restores the `officeDocument` relationship base the package declares.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn relationship_base_inverse(base: &DocxSnapshot) -> Vec<DocxStrictMutation> {
    declared_relationship_base(base, RELATIONSHIP_NAMESPACES).map(|target| DocxStrictMutation::SetRelationshipBase(set_relationship_base::SetRelationshipBase { base: target })).into_iter().collect()
}

/// ↩️ The mutation that restores the main part's `conformance` attribute: its value, or its absence.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn conformance_attribute_inverse(base: &DocxSnapshot, forward_changes: bool) -> Vec<DocxStrictMutation> {
    match conformance_attribute(base) {
        Some(value) => vec![DocxStrictMutation::SetConformanceAttribute(set_conformance_attribute::SetConformanceAttribute { value })],
        None if forward_changes => vec![DocxStrictMutation::RemoveConformanceAttribute(remove_conformance_attribute::RemoveConformanceAttribute {})],
        None => Vec::new(),
    }
}
/// ↩️ The mutation that undoes inserting a VML part: its removal, unless the part already existed.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn insert_vml_part_inverse(base: &DocxSnapshot, path: &str) -> Vec<DocxStrictMutation> {
    let key = path.trim_start_matches('/');
    if base.xml_part(key).is_some() || base.opc.part(key).is_some() {
        return Vec::new();
    }
    vec![DocxStrictMutation::RemoveVmlPart(remove_vml_part::RemoveVmlPart { path: path.to_string() })]
}

/// ↩️ The mutation that undoes removing a VML part: its insertion at the part and override positions it held.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn remove_vml_part_inverse(base: &DocxSnapshot, path: &str) -> Result<Vec<DocxStrictMutation>, semio_framework_value::ValueError> {
    let key = path.trim_start_matches('/');
    let (Some(part), Some((index, override_index))) = (base.xml_part(key), crate::standards::v_ecma_376::subsets::base::schema::mutations::xml_part_positions(base, key)) else { return Ok(Vec::new()) };
    Ok(vec![DocxStrictMutation::InsertVmlPart(insert_vml_part::InsertVmlPart { path: path.to_string(), document: part.materialize_document_exact()?, index: Some(index), override_index })])
}

/// ↩️ The mutation that undoes inserting a fallback: removing exactly the child it landed on.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn insert_alternate_content_inverse(base: &DocxSnapshot, path: &str, index: Option<usize>) -> Vec<DocxStrictMutation> {
    let Some(document) = base.xml_part(path).and_then(parse_part) else { return Vec::new() };
    vec![DocxStrictMutation::RemoveAlternateContent(remove_alternate_content::RemoveAlternateContent { path: path.to_string(), index: Some(alternate_content_landing(&document, index)) })]
}

/// ↩️ The mutations that undo removing fallbacks: one insertion of each removed node at its index, listed last-to-first because replay applies them in reverse.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn remove_alternate_content_inverse(base: &DocxSnapshot, path: &str, index: Option<usize>) -> Vec<DocxStrictMutation> {
    let Some(document) = base.xml_part(path).and_then(parse_part) else { return Vec::new() };
    alternate_content_positions(&document, index)
        .into_iter()
        .rev()
        .map(|(position, node)| DocxStrictMutation::InsertAlternateContent(insert_alternate_content::InsertAlternateContent { path: path.to_string(), node: Some(node), index: Some(position) }))
        .collect()
}
//#endregion 🔖️Inverses

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
