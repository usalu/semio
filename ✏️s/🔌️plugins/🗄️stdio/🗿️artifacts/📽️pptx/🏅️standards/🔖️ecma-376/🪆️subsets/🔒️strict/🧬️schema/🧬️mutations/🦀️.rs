//! 🧬️ `PptxStrictMutation` — the ISO/IEC 29500-1 Strict CONFORMANCE-CLASS vocabulary of
//! `stdio.pptx`. Every variant's `diff()` is handcrafted (never apply-and-capture) and every
//! variant's `inverse()` is handcrafted, reading whatever pre-state it needs out of the base. Every diff is built declaratively from the payload and
//! reads of `base`.
//!
//! **Why this subset needs a vocabulary of its own.** `🧱️base` owns the DOCUMENT vocabulary —
//! the slide, shape, paragraph and run kinds. Not one of those mutations can move a package between conformance
//! classes, because a conformance class is a property of the OPC PACKAGE and of no document object
//! at all. `check_strict_conformance` reads six axes on an already-decoded `PptxSnapshot`, one of which the `🔒️strict` DOCX subset does not have: besides the Strict PresentationML main namespace, the Transitional namespace, VML, the `officeDocument` relationship base, `conformance="strict"` and `mc:AlternateContent`, it separately rejects the Transitional DrawingML namespace. This enum is one variant per axis, plus the two baseline variants.
//!
//! **Where a PPTX package keeps its parts, and why that matters here.** Unlike `📕️xlsx` and
//! `📜️docx`, `PptxSnapshot` holds every XML part as a TYPED `PptxXmlPart` in `xml_parts`, with
//! `opc.parts` carrying only the binary ones — `encode_pptx` rejects a package that stores an XML
//! part as opaque OPC bytes. Every variant below therefore edits `xml_parts` (through the keyed
//! `PptxDiff::xml_parts` triple) and touches `opc` only for `[Content_Types].xml` and the relationship table.
//!
//! @see ../../🔣️oracle.json — the mutation catalog `KINDS` is measured against.
//! @see ../🦀️.rs — this subset's conformance check, one axis per variant below.

use crate::standards::v_ecma_376::subsets::base::schema::diff::PptxDiff;
use crate::standards::v_ecma_376::subsets::base::schema::mutations::{insert_xml_part_diff, remove_xml_part_diff, retarget_attribute_values_diff, root_attribute_diff, root_children_diff, root_edits_diff, xml_part_positions};
use crate::standards::v_ecma_376::subsets::base::schema::snapshot::{PptxSnapshot, PptxXmlPart};
use protocol::Mutation;
use semio_s_artifact_stdio_xml::schema::snapshot::{XmlAttr, XmlDocument, XmlNode};
use semio_s_artifact_stdio_zip::opc::resolve_relationship_target;
use semio_s_artifact_stdio_zip::opc::diff::{OpcDiff, OpcOwnerModification, OpcOwnerPatch, OpcOwnersDelta, OpcRelationshipModification, OpcRelationshipPatch, OpcRelationshipsDelta};

//#region 🔖️Dialect
/// 🏷️ ISO/IEC 29500-4 Transitional PresentationML main namespace.
pub const TRANSITIONAL_MAIN_NS: &str = "http://schemas.openxmlformats.org/presentationml/2006/main";
/// 🏷️ ISO/IEC 29500-1 Strict PresentationML main namespace.
pub const STRICT_MAIN_NS: &str = "http://purl.oclc.org/ooxml/presentationml/main";
/// 🏷️ The main-markup pair, `[transitional, strict]` — the order that makes the class stamp
/// bijective and therefore exactly invertible.
pub const MAIN_NAMESPACES: [&str; 2] = [TRANSITIONAL_MAIN_NS, STRICT_MAIN_NS];
/// 🎨️ ISO/IEC 29500-4 Transitional DrawingML namespace.
pub const TRANSITIONAL_DRAWING_NS: &str = "http://schemas.openxmlformats.org/drawingml/2006/main";
/// 🎨️ ISO/IEC 29500-1 Strict DrawingML namespace.
pub const STRICT_DRAWING_NS: &str = "http://purl.oclc.org/ooxml/drawingml/main";
/// 🎨️ The DrawingML pair, `[transitional, strict]` — a second namespace family a deck carries
/// independently of its PresentationML one.
pub const DRAWING_NAMESPACES: [&str; 2] = [TRANSITIONAL_DRAWING_NS, STRICT_DRAWING_NS];
/// 🔗️ ISO/IEC 29500-4 Transitional `officeDocument` relationships namespace and relationship base.
pub const TRANSITIONAL_REL: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";
/// 🔗️ ISO/IEC 29500-1 Strict `officeDocument` relationships namespace and relationship base.
pub const STRICT_REL: &str = "http://purl.oclc.org/ooxml/officeDocument/relationships";
/// 🔗️ The `officeDocument` relationships pair, `[transitional, strict]`.
pub const RELATIONSHIP_NAMESPACES: [&str; 2] = [TRANSITIONAL_REL, STRICT_REL];

/// 🧩️ The content type a legacy VML drawing part resolves. `pptx_part_is_xml` classifies `.vml` as
/// XML, so an inserted VML part belongs in `xml_parts`, never in `opc.parts` — `encode_pptx` refuses
/// a package that stores an XML part as opaque OPC bytes.
pub const VML_CONTENT_TYPE: &str = "application/vnd.openxmlformats-officedocument.vmlDrawing";

/// 🧩️ The markup-compatibility namespace an `mc:AlternateContent` fallback declares.
pub const MARKUP_COMPATIBILITY_NS: &str = "http://schemas.openxmlformats.org/markup-compatibility/2006";
/// 🧩️ The element name a markup-compatibility fallback carries.
pub const ALTERNATE_CONTENT_ELEMENT: &str = "mc:AlternateContent";
//#endregion 🔖️Dialect

//#region 🔖️Mutations
#[path = "🔀️insert-alternate-content/🦀️.rs"]
pub mod insert_alternate_content;
#[path = "🖼️insert-vml-part/🦀️.rs"]
pub mod insert_vml_part;
#[path = "🚫️remove-alternate-content/🦀️.rs"]
pub mod remove_alternate_content;
#[path = "🏷️remove-conformance-attribute/🦀️.rs"]
pub mod remove_conformance_attribute;
#[path = "🗑️remove-vml-part/🦀️.rs"]
pub mod remove_vml_part;
#[path = "🔖️set-conformance-attribute/🦀️.rs"]
pub mod set_conformance_attribute;
#[path = "🎨️set-drawing-namespace/🦀️.rs"]
pub mod set_drawing_namespace;
#[path = "🏛️set-main-namespace/🦀️.rs"]
pub mod set_main_namespace;
#[path = "🔗️set-relationship-base/🦀️.rs"]
pub mod set_relationship_base;
/// 📐️ Typed conformance-class mutation for `stdio.pptx` under ISO/IEC 29500-1
/// Strict. Every variant addresses ONE axis of the class; none addresses document content.
//#region 🔖️Leaves
//#endregion 🔖️Leaves

/// 📐️ Typed mutation for this subset. `NoMutation` was dropped: `#[derive(dsl::Mutations)]` requires
/// every variant to wrap exactly one leaf payload and a unit variant wraps none.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[mutations(snapshot = PptxSnapshot, diff = PptxDiff, schema = "PptxStrictMutation")]
pub enum PptxStrictMutation {
    SetMainNamespace(set_main_namespace::SetMainNamespace),
    SetDrawingNamespace(set_drawing_namespace::SetDrawingNamespace),
    SetRelationshipBase(set_relationship_base::SetRelationshipBase),
    SetConformanceAttribute(set_conformance_attribute::SetConformanceAttribute),
    RemoveConformanceAttribute(remove_conformance_attribute::RemoveConformanceAttribute),
    InsertVmlPart(insert_vml_part::InsertVmlPart),
    RemoveVmlPart(remove_vml_part::RemoveVmlPart),
    InsertAlternateContent(insert_alternate_content::InsertAlternateContent),
    RemoveAlternateContent(remove_alternate_content::RemoveAlternateContent),
}

/// 🧾️ Kebab-case spelling of every `PptxStrictMutation` variant, in declaration order — the exhaustive
/// mutation catalog `pptx-ecma-376-strict` (`../../🔣️oracle.json`) is measured against
/// this exact list. `kinds_match_enum_and_catalog` proves it never drifts from either side.
pub const KINDS: &[&str] =
    &["set-main-namespace", "set-drawing-namespace", "set-relationship-base", "set-conformance-attribute", "remove-conformance-attribute", "insert-vml-part", "remove-vml-part", "insert-alternate-content", "remove-alternate-content"];
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot` through its own diff — the diff is the single semantics
/// source, never a separate imperative apply path.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
#[cfg(test)]
pub fn apply_pptx_strict_mutation(snapshot: &mut PptxSnapshot, mutation: &PptxStrictMutation) -> protocol::MutationOutcome<PptxDiff> {
    let outcome = Mutation::diff(mutation, snapshot);
    match protocol::apply_diff(outcome.diff(), snapshot) {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::fatal(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}
/// 🏅️ The concrete mutations that move a deck into (`strict`) or out of the strict conformance class: both namespace families, the
/// `officeDocument` relationship base and the main part's `conformance` attribute, each through its own kind.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn stamp_conformance_class_mutations(strict: bool) -> Vec<PptxStrictMutation> {
    let index = usize::from(strict);
    vec![
        PptxStrictMutation::SetMainNamespace(set_main_namespace::SetMainNamespace { namespace: MAIN_NAMESPACES[index].to_string() }),
        PptxStrictMutation::SetDrawingNamespace(set_drawing_namespace::SetDrawingNamespace { namespace: DRAWING_NAMESPACES[index].to_string() }),
        PptxStrictMutation::SetRelationshipBase(set_relationship_base::SetRelationshipBase { base: RELATIONSHIP_NAMESPACES[index].to_string() }),
        if strict {
            PptxStrictMutation::SetConformanceAttribute(set_conformance_attribute::SetConformanceAttribute { value: "strict".to_string() })
        } else {
            PptxStrictMutation::RemoveConformanceAttribute(remove_conformance_attribute::RemoveConformanceAttribute {})
        },
    ]
}
//#endregion 🔖️Apply

//#region 🔖️Helpers
/// 🧭️ The main part's path, resolved through the root `officeDocument` relationship by type SUFFIX
/// so it resolves under either conformance class.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn main_part_path(base: &PptxSnapshot) -> Option<String> {
    let relationship = base.opc.relationships_for("").iter().find(|relationship| relationship.rel_type.ends_with("/officeDocument"))?;
    Some(resolve_relationship_target("", &relationship.target))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn xml_part<'a>(base: &'a PptxSnapshot, path: &str) -> Option<&'a PptxXmlPart> {
    let key = path.trim_start_matches('/');
    base.xml_parts.iter().find(|part| part.path == key)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn declares_namespace(node: &XmlNode, value: &str) -> bool {
    let XmlNode::Element { attrs, children, .. } = node else { return false };
    attrs.iter().any(|attr| attr.value == value) || children.iter().any(|child| declares_namespace(child, value))
}

/// 🔎️ Which member of a `[transitional, strict]` pair the deck actually declares.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn declared_pair_member(base: &PptxSnapshot, pair: [&str; 2]) -> Option<String> {
    pair.into_iter().find(|candidate| base.xml_parts.iter().any(|part| part.document.root.as_ref().is_some_and(|root| declares_namespace(root, candidate)))).map(str::to_string)
}

/// 🔎️ The relationship-type base the deck's own relationships are built on.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn declared_relationship_base(base: &PptxSnapshot, pair: [&str; 2]) -> Option<String> {
    pair.into_iter().find(|candidate| base.opc.relationships.groups().map(|(_, relationships)| relationships).flatten().any(|relationship| relationship.rel_type.starts_with(candidate))).map(str::to_string)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn root_attribute(document: &XmlDocument, name: &str) -> Option<String> {
    let XmlNode::Element { attrs, .. } = document.root.as_ref()? else { return None };
    attrs.iter().find(|attr| attr.name == name).map(|attr| attr.value.clone())
}

/// 🔎️ The main part's root `conformance` attribute, if it declares one.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn conformance_attribute(base: &PptxSnapshot) -> Option<String> {
    root_attribute(&xml_part(base, &main_part_path(base)?)?.document, "conformance")
}
//#endregion 🔖️Helpers

//#region 🔖️DiffBuilders
/// 🔺️ The diff of retargeting one namespace family across every XML part that declares it.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_retarget_namespace(base: &PptxSnapshot, from: [&str; 2], to: &str) -> PptxDiff {
    root_edits_diff(base.xml_parts.iter().filter_map(|part| retarget_attribute_values_diff(part.document.root.as_ref()?, &from, to).map(|edit| (part.path.clone(), edit))).collect())
}

/// 🔺️ The diff of retargeting the `officeDocument` relationship TYPE base, owner by owner.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_retarget_relationship_base(base: &PptxSnapshot, from: [&str; 2], to: &str) -> PptxDiff {
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
        return PptxDiff::default();
    }
    PptxDiff { schema: None, opc: Some(OpcDiff { relationships: Some(OpcOwnersDelta { modified, ..Default::default() }), ..Default::default() }), xml_parts: None }
}

/// 🔺️ The diff of setting — or removing — the main part's root `conformance` attribute.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_conformance_attribute(base: &PptxSnapshot, value: Option<&str>) -> PptxDiff {
    let Some(path) = main_part_path(base) else { return PptxDiff::default() };
    let Some(part) = xml_part(base, &path) else { return PptxDiff::default() };
    root_attribute_diff(&part.document, "conformance", value).map(|edit| root_edits_diff(vec![(part.path.clone(), edit)])).unwrap_or_default()
}
/// 🔺️ The diff of adding a legacy VML drawing part, at `index` among the XML parts and with its override at `override_index` (each appended when `None`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_insert_vml_part(base: &PptxSnapshot, path: &str, document: &XmlDocument, index: Option<usize>, override_index: Option<usize>) -> PptxDiff {
    insert_xml_part_diff(base, path, VML_CONTENT_TYPE, document, index, override_index).unwrap_or_default()
}

/// 🔺️ The diff of removing a legacy VML drawing part and its content-type override.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_remove_vml_part(base: &PptxSnapshot, path: &str) -> PptxDiff {
    remove_xml_part_diff(base, path).unwrap_or_default()
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
fn diff_insert_alternate_content(base: &PptxSnapshot, path: &str, node: Option<&XmlNode>, index: Option<usize>) -> PptxDiff {
    let Some(part) = xml_part(base, path) else { return PptxDiff::default() };
    let landing = alternate_content_landing(&part.document, index);
    root_children_diff(&part.document, Some((landing, node.cloned().unwrap_or_else(alternate_content_node))), &[]).map(|edit| root_edits_diff(vec![(part.path.clone(), edit)])).unwrap_or_default()
}

/// 🔺️ The diff of stripping the markup-compatibility fallbacks from a part's root element: every one, or only the one at `index`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_remove_alternate_content(base: &PptxSnapshot, path: &str, index: Option<usize>) -> PptxDiff {
    let Some(part) = xml_part(base, path) else { return PptxDiff::default() };
    let removed: Vec<usize> = alternate_content_positions(&part.document, index).into_iter().map(|(position, _)| position).collect();
    root_children_diff(&part.document, None, &removed).map(|edit| root_edits_diff(vec![(part.path.clone(), edit)])).unwrap_or_default()
}
//#endregion 🔖️DiffBuilders

//#region 🔖️Inverses
/// ↩️ The mutation that restores the main namespace the deck declares.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn namespace_inverse(base: &PptxSnapshot) -> Vec<PptxStrictMutation> {
    declared_pair_member(base, MAIN_NAMESPACES).map(|namespace| PptxStrictMutation::SetMainNamespace(set_main_namespace::SetMainNamespace { namespace })).into_iter().collect()
}

/// ↩️ The mutation that restores the DrawingML namespace the deck declares.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn drawing_namespace_inverse(base: &PptxSnapshot) -> Vec<PptxStrictMutation> {
    declared_pair_member(base, DRAWING_NAMESPACES).map(|namespace| PptxStrictMutation::SetDrawingNamespace(set_drawing_namespace::SetDrawingNamespace { namespace })).into_iter().collect()
}

/// ↩️ The mutation that restores the `officeDocument` relationship base the deck declares.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn relationship_base_inverse(base: &PptxSnapshot) -> Vec<PptxStrictMutation> {
    declared_relationship_base(base, RELATIONSHIP_NAMESPACES).map(|target| PptxStrictMutation::SetRelationshipBase(set_relationship_base::SetRelationshipBase { base: target })).into_iter().collect()
}

/// ↩️ The mutation that restores the main part's `conformance` attribute: its value, or its absence.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn conformance_attribute_inverse(base: &PptxSnapshot, forward_changes: bool) -> Vec<PptxStrictMutation> {
    match conformance_attribute(base) {
        Some(value) => vec![PptxStrictMutation::SetConformanceAttribute(set_conformance_attribute::SetConformanceAttribute { value })],
        None if forward_changes => vec![PptxStrictMutation::RemoveConformanceAttribute(remove_conformance_attribute::RemoveConformanceAttribute {})],
        None => Vec::new(),
    }
}

/// ↩️ The mutation that undoes inserting a VML part: its removal, unless the part already existed.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn insert_vml_part_inverse(base: &PptxSnapshot, path: &str) -> Vec<PptxStrictMutation> {
    let key = path.trim_start_matches('/');
    if base.xml_parts.iter().any(|part| part.path == key) || base.opc.part(key).is_some() {
        return Vec::new();
    }
    vec![PptxStrictMutation::RemoveVmlPart(remove_vml_part::RemoveVmlPart { path: path.to_string() })]
}

/// ↩️ The mutation that undoes removing a VML part: its insertion at the part and override positions it held.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn remove_vml_part_inverse(base: &PptxSnapshot, path: &str) -> Vec<PptxStrictMutation> {
    let (Some(part), Some((index, override_index))) = (xml_part(base, path), xml_part_positions(base, path)) else { return Vec::new() };
    vec![PptxStrictMutation::InsertVmlPart(insert_vml_part::InsertVmlPart { path: path.to_string(), document: part.document.clone(), index: Some(index), override_index })]
}

/// ↩️ The mutation that undoes inserting a fallback: removing exactly the child it landed on.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn insert_alternate_content_inverse(base: &PptxSnapshot, path: &str, index: Option<usize>) -> Vec<PptxStrictMutation> {
    let Some(part) = xml_part(base, path) else { return Vec::new() };
    vec![PptxStrictMutation::RemoveAlternateContent(remove_alternate_content::RemoveAlternateContent { path: path.to_string(), index: Some(alternate_content_landing(&part.document, index)) })]
}

/// ↩️ The mutations that undo removing fallbacks: one insertion of each removed node at its index, listed last-to-first because replay applies them in reverse.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn remove_alternate_content_inverse(base: &PptxSnapshot, path: &str, index: Option<usize>) -> Vec<PptxStrictMutation> {
    let Some(part) = xml_part(base, path) else { return Vec::new() };
    alternate_content_positions(&part.document, index)
        .into_iter()
        .rev()
        .map(|(position, node)| PptxStrictMutation::InsertAlternateContent(insert_alternate_content::InsertAlternateContent { path: path.to_string(), node: Some(node), index: Some(position) }))
        .collect()
}
//#endregion 🔖️Inverses

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
