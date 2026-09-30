//! 🧬️ `DocxTransitionalMutation` — the ISO/IEC 29500-4 Transitional CONFORMANCE-CLASS vocabulary of
//! `stdio.docx`. Every variant's `diff()` is handcrafted (never apply-and-capture) and every
//! variant's `inverse()` is handcrafted, reading whatever pre-state it needs out of the base.
//!
//! **Why this subset needs a vocabulary of its own.** `✳️any` owns the DOCUMENT vocabulary —
//! `insert-block`, `remove-block`, `set-block-content`, `set-run-text`, the style kinds and the part kinds. Not one of those mutations can move a
//! package between conformance classes, because a conformance class is a property of the OPC
//! PACKAGE and of no document object at all. `check_transitional_conformance` reads three axes: the main document part's Transitional WordprocessingML namespace, any strict-family (`purl.oclc.org/ooxml`) namespace in a part or a relationship type, and a root `conformance="strict"` that would contradict the stamp. VML and `mc:AlternateContent` are legal Transitional markup and are not policed, which is why this enum declares four variants fewer than its `📏️strict` sibling.
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

use crate::standards::v_ecma_376::subsets::base::schema::diff::{DocxDiff, DocxOpcContentTypesDiff, DocxOpcDiff, DocxOpcPartsDiff, DocxOpcRelDiff, DocxOpcRelListDiff, DocxOpcRelationshipsDiff, NamedModified};
use crate::standards::v_ecma_376::subsets::base::schema::snapshot::{DocxSnapshot, DocxXmlPart};
use protocol::command::DiffAlgebra;
use protocol::Mutation;
use semio_s_artifact_stdio_xml::schema::snapshot::{XmlAttr, XmlDocument, XmlNode};
use semio_s_artifact_stdio_zip::opc::resolve_relationship_target;

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
//#endregion 🔖️Dialect

//#region 🔖️Mutations
#[path = "🚫️remove-conformance-attribute/🦀️.rs"]
pub mod remove_conformance_attribute;
#[path = "✅️set-conformance-attribute/🦀️.rs"]
pub mod set_conformance_attribute;
#[path = "🌐️set-main-namespace/🦀️.rs"]
pub mod set_main_namespace;
#[path = "🔗️set-relationship-base/🦀️.rs"]
pub mod set_relationship_base;
/// 📐️ Typed conformance-class mutation for `stdio.docx` under ISO/IEC 29500-4
/// Transitional. Every variant addresses ONE axis of the class; none addresses document content.
//#region 🔖️Leaves
#[path = "📸️set-snapshot/🦀️.rs"]
pub mod set_snapshot;
//#endregion 🔖️Leaves

/// 📐️ Typed mutation for this subset. `NoMutation` was dropped: `#[derive(dsl::Mutations)]` requires
/// every variant to wrap exactly one leaf payload and a unit variant wraps none.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[mutations(snapshot = DocxSnapshot, diff = DocxDiff, schema = "DocxTransitionalMutation")]
pub enum DocxTransitionalMutation {
    SetSnapshot(set_snapshot::SetSnapshot),
    SetMainNamespace(set_main_namespace::SetMainNamespace),
    SetRelationshipBase(set_relationship_base::SetRelationshipBase),
    SetConformanceAttribute(set_conformance_attribute::SetConformanceAttribute),
    RemoveConformanceAttribute(remove_conformance_attribute::RemoveConformanceAttribute),
}

/// 🧾️ Kebab-case spelling of every `DocxTransitionalMutation` variant, in declaration order — the exhaustive
/// mutation catalog `docx-ecma-376-transitional` (`../../🔣️oracle.json`) is measured against
/// this exact list. `kinds_match_enum_and_catalog` proves it never drifts from either side.
pub const KINDS: &[&str] = &["set-snapshot", "set-main-namespace", "set-relationship-base", "set-conformance-attribute", "remove-conformance-attribute"];
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot` through its own diff — the diff is the single semantics
/// source, never a separate imperative apply path.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_docx_transitional_mutation(snapshot: &mut DocxSnapshot, mutation: &DocxTransitionalMutation) -> protocol::MutationOutcome<DocxDiff> {
    let outcome = Mutation::diff(mutation, snapshot);
    match protocol::MutationDiff::apply(outcome.diff(), snapshot) {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::fatal(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}

/// 🏅️ The one whole-package operation that moves `base` into (`strict`) or out of the strict conformance class: a
/// `set-snapshot` of [`stamp_conformance_class`]'s stamp — what a class conversion records as one edit.
pub fn stamp_conformance_class_mutation(base: &DocxSnapshot, strict: bool) -> DocxTransitionalMutation {
    DocxTransitionalMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: stamp_conformance_class(base.clone(), strict) })
}

/// 📨️ Builds the operation of semantic kind `kind` from its editable payload JSON — the leaf wire (`payload_value()`) a
/// `🥒️.feature` row carries — through the derive's generic `from_payload_value`.
pub fn decode_docx_transitional_mutation_payload(kind: &str, payload: &str) -> Result<DocxTransitionalMutation, String> {
    protocol::os_pack::from_json_str(payload).and_then(|value| <DocxTransitionalMutation as Mutation<DocxSnapshot>>::from_payload_value(kind, value)).map_err(|error| error.to_string())
}

/// 🔙️ The operations that undo `mutation` on `base` — the aggregate's own leaf-owned `Mutation::inverse`, the law a
/// case's inverse scenario holds this implementation to.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse_docx_transitional_mutation(mutation: &DocxTransitionalMutation, base: &DocxSnapshot) -> Vec<DocxTransitionalMutation> {
    <DocxTransitionalMutation as Mutation<DocxSnapshot>>::inverse(mutation, base)
}
//#endregion 🔖️Apply

//#region 🔖️Helpers
/// 🧭️ The main part's path, resolved through the root `officeDocument` relationship by type SUFFIX
/// so it resolves under either conformance class — matching by the transitional-shaped constant
/// verbatim would silently fail to find the main part of a genuinely Strict package.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn main_part_path(base: &DocxSnapshot) -> Option<String> {
    let relationship = base.opc.relationships_for("").iter().find(|relationship| relationship.rel_type.ends_with("/officeDocument"))?;
    Some(resolve_relationship_target("", &relationship.target))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_part(part: &DocxXmlPart) -> Option<XmlDocument> {
    Some(part.document.clone())
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

/// 🔎️ Which member of a `[transitional, strict]` pair the package actually declares.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn declared_pair_member(base: &DocxSnapshot, pair: [&str; 2]) -> Option<String> {
    pair.into_iter().find(|candidate| base.xml_parts.iter().filter_map(parse_part).any(|document| document.root.as_ref().is_some_and(|root| declares_namespace(root, candidate)))).map(str::to_string)
}

/// 🔎️ The relationship-type base the package's own relationships are built on.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn declared_relationship_base(base: &DocxSnapshot, pair: [&str; 2]) -> Option<String> {
    pair.into_iter().find(|candidate| base.opc.relationships.values().flatten().any(|relationship| relationship.rel_type.starts_with(candidate))).map(str::to_string)
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

/// 🏅️ Stamps a whole snapshot into one conformance class: both namespace families, the
/// `officeDocument` relationship base, and the main part's own `conformance` attribute. Bijective by
/// construction, so stamping back is an exact inverse — which is what makes `SetSnapshot` invertible
/// on this axis.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn stamp_conformance_class(mut snapshot: DocxSnapshot, strict: bool) -> DocxSnapshot {
    let index = usize::from(strict);
    for part in &mut snapshot.xml_parts {
        let Some(root) = part.document.root.as_mut() else { continue };
        retarget_namespace(root, &MAIN_NAMESPACES, MAIN_NAMESPACES[index]);
        retarget_namespace(root, &RELATIONSHIP_NAMESPACES, RELATIONSHIP_NAMESPACES[index]);
    }
    for relationships in snapshot.opc.relationships.values_mut() {
        for relationship in relationships {
            let Some(prefix) = RELATIONSHIP_NAMESPACES.into_iter().find(|prefix| relationship.rel_type.starts_with(prefix)) else { continue };
            relationship.rel_type = format!("{}{}", RELATIONSHIP_NAMESPACES[index], &relationship.rel_type[prefix.len()..]);
        }
    }
    if let Some(path) = main_part_path(&snapshot) {
        if let Some(part) = snapshot.xml_part_mut(&path) {
            set_root_attribute(&mut part.document, "conformance", strict.then_some("strict"));
        }
    }
    snapshot
}
//#endregion 🔖️Helpers

//#region 🔖️DiffBuilders
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn opc_diff(parts: Option<DocxOpcPartsDiff>, content_types: Option<DocxOpcContentTypesDiff>, relationships: Option<DocxOpcRelationshipsDiff>) -> DocxDiff {
    if parts.is_none() && content_types.is_none() && relationships.is_none() {
        return DocxDiff::default();
    }
    DocxDiff { opc: Some(DocxOpcDiff { content_types, parts, relationships, comment: None }), ..Default::default() }
}

/// 🔺️ The diff of retargeting one namespace family across every XML part that declares it.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_retarget_namespace(base: &DocxSnapshot, from: [&str; 2], to: &str) -> DocxDiff {
    let mut next = base.clone();
    for part in &mut next.xml_parts {
        let Some(root) = part.document.root.as_mut() else { continue };
        retarget_namespace(root, &from, to);
    }
    <DocxDiff as DiffAlgebra<DocxSnapshot>>::between(base, &next)
}

/// 🔺️ The diff of retargeting the `officeDocument` relationship TYPE base, owner by owner.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_retarget_relationship_base(base: &DocxSnapshot, from: [&str; 2], to: &str) -> DocxDiff {
    let mut owners: Vec<&String> = base.opc.relationships.keys().collect();
    owners.sort();
    let mut modified = Vec::new();
    for owner in owners {
        let mut entries = Vec::new();
        for relationship in &base.opc.relationships[owner] {
            let Some(prefix) = from.into_iter().find(|prefix| relationship.rel_type.starts_with(prefix)) else { continue };
            let retargeted = format!("{to}{}", &relationship.rel_type[prefix.len()..]);
            if retargeted == relationship.rel_type {
                continue;
            }
            entries.push(NamedModified { key: relationship.id.clone(), diff: DocxOpcRelDiff { rel_type: Some(retargeted), target: None, target_mode: None } });
        }
        if entries.is_empty() {
            continue;
        }
        modified.push(NamedModified { key: owner.clone(), diff: DocxOpcRelListDiff { modified: entries, ..Default::default() } });
    }
    if modified.is_empty() {
        return DocxDiff::default();
    }
    opc_diff(None, None, Some(DocxOpcRelationshipsDiff { modified, ..Default::default() }))
}

/// 🔺️ The diff of setting — or removing — the main part's root `conformance` attribute.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_conformance_attribute(base: &DocxSnapshot, value: Option<&str>) -> DocxDiff {
    let Some(path) = main_part_path(base) else { return DocxDiff::default() };
    let mut next = base.clone();
    let Some(part) = next.xml_part_mut(&path) else { return DocxDiff::default() };
    if !set_root_attribute(&mut part.document, "conformance", value) {
        return DocxDiff::default();
    }
    <DocxDiff as DiffAlgebra<DocxSnapshot>>::between(base, &next)
}
//#endregion 🔖️DiffBuilders

//#region 🔖️MutationTrait
// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_diff(this: &DocxTransitionalMutation, base: &DocxSnapshot) -> protocol::MutationOutcome<DocxDiff> {
    protocol::MutationOutcome::new(match this {
        DocxTransitionalMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => <DocxDiff as DiffAlgebra<DocxSnapshot>>::between(base, snapshot),
        DocxTransitionalMutation::SetMainNamespace(set_main_namespace::SetMainNamespace { namespace }) => diff_retarget_namespace(base, MAIN_NAMESPACES, namespace),
        DocxTransitionalMutation::SetRelationshipBase(set_relationship_base::SetRelationshipBase { base: target }) => diff_retarget_relationship_base(base, RELATIONSHIP_NAMESPACES, target),
        DocxTransitionalMutation::SetConformanceAttribute(set_conformance_attribute::SetConformanceAttribute { value }) => diff_conformance_attribute(base, Some(value)),
        DocxTransitionalMutation::RemoveConformanceAttribute(_) => diff_conformance_attribute(base, None),
    })
}

// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_inverse(this: &DocxTransitionalMutation, base: &DocxSnapshot) -> Vec<DocxTransitionalMutation> {
    vec![match this {
        DocxTransitionalMutation::SetSnapshot(_) => DocxTransitionalMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() }),
        DocxTransitionalMutation::SetMainNamespace(_) => match declared_pair_member(base, MAIN_NAMESPACES) {
            Some(namespace) => DocxTransitionalMutation::SetMainNamespace(set_main_namespace::SetMainNamespace { namespace }),
            None => return Vec::new(),
        },
        DocxTransitionalMutation::SetRelationshipBase(_) => match declared_relationship_base(base, RELATIONSHIP_NAMESPACES) {
            Some(target) => DocxTransitionalMutation::SetRelationshipBase(set_relationship_base::SetRelationshipBase { base: target }),
            None => return Vec::new(),
        },
        DocxTransitionalMutation::SetConformanceAttribute(_) => match conformance_attribute(base) {
            Some(value) => DocxTransitionalMutation::SetConformanceAttribute(set_conformance_attribute::SetConformanceAttribute { value }),
            None => DocxTransitionalMutation::RemoveConformanceAttribute(remove_conformance_attribute::RemoveConformanceAttribute {}),
        },
        DocxTransitionalMutation::RemoveConformanceAttribute(_) => match conformance_attribute(base) {
            Some(value) => DocxTransitionalMutation::SetConformanceAttribute(set_conformance_attribute::SetConformanceAttribute { value }),
            None => return Vec::new(),
        },
    }]
}
//#endregion 🔖️MutationTrait

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
