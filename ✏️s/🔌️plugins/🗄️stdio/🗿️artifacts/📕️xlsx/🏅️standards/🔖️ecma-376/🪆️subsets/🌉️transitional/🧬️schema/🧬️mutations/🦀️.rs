//! 🧬️ `XlsxTransitionalMutation` — the ISO/IEC 29500-4 Transitional CONFORMANCE-CLASS vocabulary of
//! `stdio.xlsx`. Every variant's `diff()` is handcrafted (never apply-and-capture) and every
//! variant's `inverse()` is handcrafted, reading whatever pre-state it needs out of the base.
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

use crate::standards::v_ecma_376::subsets::base::schema::diff::{diff_set_snapshot, XlsxDiff};
use crate::standards::v_ecma_376::subsets::base::schema::snapshot::XlsxSnapshot;
use protocol::command::DiffAlgebra;
use protocol::Mutation;
use semio_s_artifact_stdio_xml::schema::snapshot::{XmlAttr, XmlDocument, XmlNode};
use semio_s_artifact_stdio_zip::opc::resolve_relationship_target;

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
#[path = "📸️set-snapshot/🦀️.rs"]
pub mod set_snapshot;
#[path = "🏷️set-worksheet-content-type/🦀️.rs"]
pub mod set_worksheet_content_type;
//#endregion 🔖️Leaves

/// 📐️ Typed mutation for this subset. `NoMutation` was dropped: `#[derive(dsl::Mutations)]` requires
/// every variant to wrap exactly one leaf payload and a unit variant wraps none.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[mutations(snapshot = XlsxSnapshot, diff = XlsxDiff, schema = "XlsxTransitionalMutation")]
pub enum XlsxTransitionalMutation {
    SetSnapshot(set_snapshot::SetSnapshot),
    SetMainNamespace(set_main_namespace::SetMainNamespace),
    SetRelationshipsNamespace(set_relationships_namespace::SetRelationshipsNamespace),
    SetConformanceAttribute(set_conformance_attribute::SetConformanceAttribute),
    RemoveConformanceAttribute(remove_conformance_attribute::RemoveConformanceAttribute),
    SetWorksheetContentType(set_worksheet_content_type::SetWorksheetContentType),
}

/// 🧾️ Kebab-case spelling of every `XlsxTransitionalMutation` variant, in declaration order — the exhaustive
/// mutation catalog `xlsx-ecma-376-transitional` (`../../🔣️oracle.json`) is measured against
/// this exact list. `kinds_match_enum_and_catalog` proves it never drifts from either side.
pub const KINDS: &[&str] = &["set-snapshot", "set-main-namespace", "set-relationships-namespace", "set-conformance-attribute", "remove-conformance-attribute", "set-worksheet-content-type"];
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot` through its own diff — the diff is the single semantics
/// source, never a separate imperative apply path.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_xlsx_transitional_mutation(snapshot: &mut XlsxSnapshot, mutation: &XlsxTransitionalMutation) -> protocol::MutationOutcome<XlsxDiff> {
    let outcome = Mutation::diff(mutation, snapshot);
    match protocol::MutationDiff::apply(outcome.diff(), snapshot) {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::fatal(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
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

/// ✍️ Rewrites every attribute value equal to a member of `from` to `to`, through the whole
/// subtree — a namespace declaration is an ordinary attribute, which is why one walk covers
/// `xmlns`, `xmlns:r` and whatever prefixed alias a real package happens to use.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn retarget_namespace(node: &mut XmlNode, from: &[&str], to: &str) -> bool {
    let XmlNode::Element { attrs, children, .. } = node else { return false };
    let mut changed = false;
    for attr in attrs.iter_mut() {
        if from.contains(&attr.value.as_str()) && attr.value != to {
            attr.value = to.to_string();
            changed = true;
        }
    }
    for child in children.iter_mut() {
        changed |= retarget_namespace(child, from, to);
    }
    changed
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

/// ✍️ Sets — or, with `None`, removes — one attribute on the ROOT element only.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn set_root_attribute(document: &mut XmlDocument, name: &str, value: Option<&str>) -> bool {
    let Some(XmlNode::Element { attrs, .. }) = document.root.as_mut() else { return false };
    match (attrs.iter().position(|attr| attr.name == name), value) {
        (Some(index), Some(value)) => attrs[index].value = value.to_string(),
        (Some(index), None) => {
            attrs.remove(index);
        }
        (None, Some(value)) => attrs.push(XmlAttr { name: name.to_string(), value: value.to_string() }),
        (None, None) => return false,
    }
    true
}

/// 🏅️ Stamps a whole snapshot into one conformance class: both namespace families across every
/// logical XML part, the `officeDocument` relationship base, and the main part's own `conformance`
/// attribute. Bijective by construction, so stamping back is an exact inverse — which is what makes
/// `SetSnapshot` invertible on this axis.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn stamp_conformance_class(mut snapshot: XlsxSnapshot, strict: bool) -> XlsxSnapshot {
    let index = usize::from(strict);
    for part in snapshot.xml_parts.iter_mut() {
        let Some(root) = part.document.root.as_mut() else { continue };
        retarget_namespace(root, &MAIN_NAMESPACES, MAIN_NAMESPACES[index]);
        retarget_namespace(root, &RELATIONSHIP_NAMESPACES, RELATIONSHIP_NAMESPACES[index]);
    }
    for relationships in snapshot.opc.relationships.groups_mut().map(|(_, relationships)| relationships) {
        for relationship in relationships.iter_mut() {
            let Some(prefix) = RELATIONSHIP_NAMESPACES.into_iter().find(|prefix| relationship.rel_type.starts_with(prefix)) else { continue };
            relationship.rel_type = format!("{}{}", RELATIONSHIP_NAMESPACES[index], &relationship.rel_type[prefix.len()..]);
        }
    }
    if let Some(path) = main_part_path(&snapshot) {
        if let Some(part) = snapshot.xml_part_mut(&path) {
            set_root_attribute(&mut part.document, "conformance", if strict { Some("strict") } else { None });
        }
    }
    snapshot
}

/// 🏅️ The one whole-package operation that moves `base` into (`strict`) or out of the strict conformance class: a
/// `set-snapshot` of [`stamp_conformance_class`]'s stamp — what a class conversion records as one edit.
pub fn stamp_conformance_class_mutation(base: &XlsxSnapshot, strict: bool) -> XlsxTransitionalMutation {
    XlsxTransitionalMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: stamp_conformance_class(base.clone(), strict) })
}
//#endregion 🔖️Helpers

//#region 🔖️DiffBuilders
/// 🔺️ The sparse diff carrying `base` to its edited copy `next`, through the base subset's own
/// `diff_set_snapshot` — none when the edit changed nothing.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_to(base: &XlsxSnapshot, next: XlsxSnapshot) -> XlsxDiff {
    if next == *base {
        return XlsxDiff::default();
    }
    diff_set_snapshot(base, &next)
}

/// 🔺️ The diff of retargeting one namespace family across every logical XML part that declares it.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_retarget_namespace(base: &XlsxSnapshot, from: [&str; 2], to: &str) -> XlsxDiff {
    let mut next = base.clone();
    for part in next.xml_parts.iter_mut() {
        if let Some(root) = part.document.root.as_mut() {
            retarget_namespace(root, &from, to);
        }
    }
    diff_to(base, next)
}

/// 🔺️ The diff of setting — or removing — the main part's root `conformance` attribute.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_conformance_attribute(base: &XlsxSnapshot, value: Option<&str>) -> XlsxDiff {
    let Some(path) = main_part_path(base) else { return XlsxDiff::default() };
    let mut next = base.clone();
    let Some(part) = next.xml_part_mut(&path) else { return XlsxDiff::default() };
    set_root_attribute(&mut part.document, "conformance", value);
    diff_to(base, next)
}

/// 🔺️ The diff of retyping one part: the logical XML part's own `content_type` and its
/// `[Content_Types].xml` override move together, because the two can never be allowed to drift apart.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_set_content_type(base: &XlsxSnapshot, path: &str, content_type: &str) -> XlsxDiff {
    let mut next = base.clone();
    let Some(part) = next.xml_part_mut(path) else { return XlsxDiff::default() };
    part.content_type = content_type.to_string();
    next.opc.content_types.set_override(path, content_type);
    diff_to(base, next)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn resolved_content_type(base: &XlsxSnapshot, path: &str) -> Option<String> {
    base.opc.content_types.resolve(path).map(str::to_string)
}
//#endregion 🔖️DiffBuilders

//#region 🔖️MutationTrait
// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_diff(this: &XlsxTransitionalMutation, base: &XlsxSnapshot) -> protocol::MutationOutcome<XlsxDiff> {
    protocol::MutationOutcome::new(match this {
        XlsxTransitionalMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => <XlsxDiff as DiffAlgebra<XlsxSnapshot>>::between(base, snapshot),
        XlsxTransitionalMutation::SetMainNamespace(set_main_namespace::SetMainNamespace { namespace }) => diff_retarget_namespace(base, MAIN_NAMESPACES, namespace),
        XlsxTransitionalMutation::SetRelationshipsNamespace(set_relationships_namespace::SetRelationshipsNamespace { namespace }) => diff_retarget_namespace(base, RELATIONSHIP_NAMESPACES, namespace),
        XlsxTransitionalMutation::SetConformanceAttribute(set_conformance_attribute::SetConformanceAttribute { value }) => diff_conformance_attribute(base, Some(value)),
        XlsxTransitionalMutation::RemoveConformanceAttribute(_) => diff_conformance_attribute(base, None),
        XlsxTransitionalMutation::SetWorksheetContentType(set_worksheet_content_type::SetWorksheetContentType { path, content_type }) => diff_set_content_type(base, path, content_type),
    })
}

// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_inverse(this: &XlsxTransitionalMutation, base: &XlsxSnapshot) -> Result<Vec<XlsxTransitionalMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![match this {
        XlsxTransitionalMutation::SetSnapshot(_) => XlsxTransitionalMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() }),
        XlsxTransitionalMutation::SetMainNamespace(_) => match declared_pair_member(base, MAIN_NAMESPACES) {
            Some(namespace) => XlsxTransitionalMutation::SetMainNamespace(set_main_namespace::SetMainNamespace { namespace }),
            None => return Vec::new(),
        },
        XlsxTransitionalMutation::SetRelationshipsNamespace(_) => match declared_pair_member(base, RELATIONSHIP_NAMESPACES) {
            Some(namespace) => XlsxTransitionalMutation::SetRelationshipsNamespace(set_relationships_namespace::SetRelationshipsNamespace { namespace }),
            None => return Vec::new(),
        },
        XlsxTransitionalMutation::SetConformanceAttribute(_) => match conformance_attribute(base) {
            Some(value) => XlsxTransitionalMutation::SetConformanceAttribute(set_conformance_attribute::SetConformanceAttribute { value }),
            None => XlsxTransitionalMutation::RemoveConformanceAttribute(remove_conformance_attribute::RemoveConformanceAttribute {}),
        },
        XlsxTransitionalMutation::RemoveConformanceAttribute(_) => match conformance_attribute(base) {
            Some(value) => XlsxTransitionalMutation::SetConformanceAttribute(set_conformance_attribute::SetConformanceAttribute { value }),
            None => return Vec::new(),
        },
        XlsxTransitionalMutation::SetWorksheetContentType(set_worksheet_content_type::SetWorksheetContentType { path, .. }) => match resolved_content_type(base, path) {
            Some(content_type) => XlsxTransitionalMutation::SetWorksheetContentType(set_worksheet_content_type::SetWorksheetContentType { path: path.clone(), content_type }),
            None => return Vec::new(),
        },
    }]

    })())
}
//#endregion 🔖️MutationTrait

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
