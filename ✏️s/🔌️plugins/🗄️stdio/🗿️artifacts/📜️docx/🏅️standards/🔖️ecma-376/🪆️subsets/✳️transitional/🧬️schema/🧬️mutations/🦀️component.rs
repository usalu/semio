//! 🧬️ `DocxTransitionalMutation` — the ISO/IEC 29500-4 Transitional CONFORMANCE-CLASS vocabulary of
//! `stdio.docx`. Every variant's `diff()` is handcrafted (never apply-and-capture) and every
//! variant's `inverse()` is handcrafted, reading whatever pre-state it needs out of the base.
//!
//! **Why this subset needs a vocabulary of its own.** `✳️any` owns the DOCUMENT vocabulary —
//! `insert-block`, `remove-block`, `set-block-content`, `set-run-text`, the style kinds and the part kinds. Not one of those mutations can move a
//! package between conformance classes, because a conformance class is a property of the OPC
//! PACKAGE and of no document object at all. `check_transitional_conformance` reads three axes: the main document part's Transitional WordprocessingML namespace, any strict-family (`purl.oclc.org/ooxml`) namespace in a part or a relationship type, and a root `conformance="strict"` that would contradict the stamp. VML and `mc:AlternateContent` are legal Transitional markup and are not policed, which is why this enum declares four variants fewer than its `✳️strict` sibling.
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
//! @see ../../🧪️oracle/🔣️component.json — the mutation catalog `KINDS` is measured against.
//! @see ../🦀️component.rs — this subset's conformance check, one axis per variant below.

use crate::artifacts::docx::standards::v_ecma_376::subsets::any::schema::diff::{NamedModified, NamedTripleDiff, DocxDiff, DocxOpcContentTypesDiff, DocxOpcCtEntriesDiff, DocxOpcDiff, DocxOpcPartDiff, DocxOpcPartsDiff, DocxOpcRelDiff, DocxOpcRelListDiff, DocxOpcRelationshipsDiff};
use crate::artifacts::docx::standards::v_ecma_376::subsets::any::schema::snapshot::DocxSnapshot;
use crate::artifacts::xml::schema::snapshot::{xml_document_from_text, xml_document_to_text, XmlAttr, XmlDocument, XmlNode};
use crate::artifacts::zip::opc::{resolve_relationship_target, OpcPart};
use protocol::command::DiffAlgebra;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

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
/// 📐️ Typed conformance-class mutation for `stdio.docx` under ISO/IEC 29500-4
/// Transitional. Every variant addresses ONE axis of the class; none addresses document content.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum DocxTransitionalMutation {
    /// 🚫️ The identity element of the vocabulary.
    #[default]
    NoMutation,
    /// 🔄️ Replaces the whole package. A conformance class is a whole-package property, so this is the class stamp in its total form — every namespace family, the relationship base and the root attribute at once. Build the target with [`stamp_conformance_class`].
    SetSnapshot {
        snapshot: DocxSnapshot,
    },
    /// 🏷️ Retargets the main WordprocessingML namespace declaration wherever it is declared, from whichever member of the `[transitional, strict]` pair the package currently carries to `namespace`.
    SetMainNamespace {
        namespace: String,
    },
    /// 🔗️ Retargets the `officeDocument` relationship TYPE base of every relationship in the package, leaving the package-level relationship types (`…/package/2006/relationships/…`) both conformance classes share untouched.
    SetRelationshipBase {
        base: String,
    },
    /// 🖋️ Sets the main part's root `conformance` attribute.
    SetConformanceAttribute {
        value: String,
    },
    /// 🖋️ Removes the main part's root `conformance` attribute.
    RemoveConformanceAttribute,
}

/// 🧾️ Kebab-case spelling of every `DocxTransitionalMutation` variant, in declaration order — the exhaustive
/// mutation catalog `docx-ecma-376-transitional` (`../../🧪️oracle/🔣️component.json`) is measured against
/// this exact list. `kinds_match_enum_and_catalog` proves it never drifts from either side.
pub const KINDS: &[&str] = &["no-mutation", "set-snapshot", "set-main-namespace", "set-relationship-base", "set-conformance-attribute", "remove-conformance-attribute"];
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
        Err(error) => protocol::MutationOutcome::error(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
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

/// 📰️ Whether a part is XML this vocabulary may rewrite. `.rels` parts never appear in `opc.parts`
/// — they are decoded into `opc.relationships`, which the relationship-base axis addresses instead.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn is_xml_part(path: &str) -> bool {
    let lower = path.to_ascii_lowercase();
    lower.ends_with(".xml") || lower.ends_with(".vml")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_part(part: &OpcPart) -> Option<XmlDocument> {
    xml_document_from_text(std::str::from_utf8(&part.bytes).ok()?).ok()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn part_text(base: &DocxSnapshot, path: &str) -> Option<String> {
    String::from_utf8(base.opc.part(path)?.bytes.clone()).ok()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn serialize(document: &XmlDocument) -> Vec<u8> {
    xml_document_to_text(document).into_bytes()
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
    pair.into_iter()
        .find(|candidate| base.opc.parts.iter().filter(|part| is_xml_part(&part.path)).filter_map(parse_part).any(|document| document.root.as_ref().is_some_and(|root| declares_namespace(root, candidate))))
        .map(str::to_string)
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
    root_attribute(&parse_part(base.opc.part(&main_part_path(base)?)?)?, "conformance")
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
    for part in snapshot.opc.parts.iter_mut() {
        if !is_xml_part(&part.path) {
            continue;
        }
        let Some(mut document) = parse_part(part) else { continue };
        let Some(root) = document.root.as_mut() else { continue };
        let mut changed = retarget_namespace(root, &MAIN_NAMESPACES, MAIN_NAMESPACES[index]);
        changed |= retarget_namespace(root, &RELATIONSHIP_NAMESPACES, RELATIONSHIP_NAMESPACES[index]);
        if changed {
            part.bytes = serialize(&document);
        }
    }
    for relationships in snapshot.opc.relationships.values_mut() {
        for relationship in relationships.iter_mut() {
            let Some(prefix) = RELATIONSHIP_NAMESPACES.into_iter().find(|prefix| relationship.rel_type.starts_with(prefix)) else { continue };
            relationship.rel_type = format!("{}{}", RELATIONSHIP_NAMESPACES[index], &relationship.rel_type[prefix.len()..]);
        }
    }
    if let Some(path) = main_part_path(&snapshot) {
        if let Some(part) = snapshot.opc.part(&path).cloned() {
            if let Some(mut document) = parse_part(&part) {
                set_root_attribute(&mut document, "conformance", if strict { Some("strict") } else { None });
                snapshot.opc.set_part(&part.path, &part.content_type, serialize(&document));
            }
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
    DocxDiff { opc: Some(DocxOpcDiff { content_types, parts, relationships }), ..Default::default() }
}

/// 🔺️ Sparse per-part diff: the touched parts only, each carrying just the fields that moved.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parts_diff(modified: Vec<(String, DocxOpcPartDiff)>, added: Vec<OpcPart>, removed: Vec<String>) -> Option<DocxOpcPartsDiff> {
    if modified.is_empty() && added.is_empty() && removed.is_empty() {
        return None;
    }
    Some(NamedTripleDiff { removed, modified: modified.into_iter().map(|(key, diff)| NamedModified { key, diff }).collect(), added })
}

/// 🔺️ The diff of retargeting one namespace family across every XML part that declares it.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_retarget_namespace(base: &DocxSnapshot, from: [&str; 2], to: &str) -> DocxDiff {
    let mut modified = Vec::new();
    for part in &base.opc.parts {
        if !is_xml_part(&part.path) {
            continue;
        }
        let Some(mut document) = parse_part(part) else { continue };
        let Some(root) = document.root.as_mut() else { continue };
        if !retarget_namespace(root, &from, to) {
            continue;
        }
        modified.push((part.path.clone(), DocxOpcPartDiff { content_type: None, bytes: Some(serialize(&document)) }));
    }
    opc_diff(parts_diff(modified, Vec::new(), Vec::new()), None, None)
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
    let Some(part) = base.opc.part(&path) else { return DocxDiff::default() };
    let Some(mut document) = parse_part(part) else { return DocxDiff::default() };
    if !set_root_attribute(&mut document, "conformance", value) {
        return DocxDiff::default();
    }
    opc_diff(parts_diff(vec![(part.path.clone(), DocxOpcPartDiff { content_type: None, bytes: Some(serialize(&document)) })], Vec::new(), Vec::new()), None, None)
}
//#endregion 🔖️DiffBuilders

//#region 🔖️MutationTrait
impl Mutation<DocxSnapshot> for DocxTransitionalMutation {
    type Diff = DocxDiff;

    fn diff(&self, base: &DocxSnapshot) -> protocol::MutationOutcome<Self::Diff> {
        protocol::MutationOutcome::new(match self {
            Self::NoMutation => DocxDiff::default(),
            Self::SetSnapshot { snapshot } => <DocxDiff as DiffAlgebra<DocxSnapshot>>::between(base, snapshot),
            Self::SetMainNamespace { namespace } => diff_retarget_namespace(base, MAIN_NAMESPACES, namespace),
            Self::SetRelationshipBase { base: target } => diff_retarget_relationship_base(base, RELATIONSHIP_NAMESPACES, target),
            Self::SetConformanceAttribute { value } => diff_conformance_attribute(base, Some(value)),
            Self::RemoveConformanceAttribute => diff_conformance_attribute(base, None),
        })
    }

    fn inverse(&self, base: &DocxSnapshot) -> Vec<Self> {
        vec![match self {
            Self::NoMutation => Self::NoMutation,
            Self::SetSnapshot { .. } => Self::SetSnapshot { snapshot: base.clone() },
            Self::SetMainNamespace { .. } => match declared_pair_member(base, MAIN_NAMESPACES) {
                Some(namespace) => Self::SetMainNamespace { namespace },
                None => Self::NoMutation,
            },
            Self::SetRelationshipBase { .. } => match declared_relationship_base(base, RELATIONSHIP_NAMESPACES) {
                Some(target) => Self::SetRelationshipBase { base: target },
                None => Self::NoMutation,
            },
            Self::SetConformanceAttribute { .. } => match conformance_attribute(base) {
                Some(value) => Self::SetConformanceAttribute { value },
                None => Self::RemoveConformanceAttribute,
            },
            Self::RemoveConformanceAttribute => match conformance_attribute(base) {
                Some(value) => Self::SetConformanceAttribute { value },
                None => Self::NoMutation,
            },
        }]
    }
}
//#endregion 🔖️MutationTrait

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    //#region 🔖️KindsConformanceLaw
    /// 🧭️ `kind_of` is an EXHAUSTIVE match (no wildcard arm) — the compiler refuses this file if a
    /// variant is added to `DocxTransitionalMutation` without a matching kebab-case spelling here, which is what keeps
    /// `KINDS` honest against the enum. The second half reads the sibling oracle manifest's `kinds`
    /// array as text (the framework never parses Rust, so this is the only side that can prove the
    /// manifest matches) and asserts the same list, in the same order.
    #[test]
    fn kinds_match_enum_and_catalog() {
        fn kind_of(mutation: &DocxTransitionalMutation) -> &'static str {
            match mutation {
                DocxTransitionalMutation::NoMutation => "no-mutation",
                DocxTransitionalMutation::SetSnapshot { .. } => "set-snapshot",
                DocxTransitionalMutation::SetMainNamespace { .. } => "set-main-namespace",
                DocxTransitionalMutation::SetRelationshipBase { .. } => "set-relationship-base",
                DocxTransitionalMutation::SetConformanceAttribute { .. } => "set-conformance-attribute",
                DocxTransitionalMutation::RemoveConformanceAttribute => "remove-conformance-attribute",
            }
        }
        let samples = [
            DocxTransitionalMutation::NoMutation,
            DocxTransitionalMutation::SetSnapshot { snapshot: DocxSnapshot::default() },
            DocxTransitionalMutation::SetMainNamespace { namespace: String::new() },
            DocxTransitionalMutation::SetRelationshipBase { base: String::new() },
            DocxTransitionalMutation::SetConformanceAttribute { value: String::new() },
            DocxTransitionalMutation::RemoveConformanceAttribute,
        ];
        let from_enum: Vec<&'static str> = samples.iter().map(kind_of).collect();
        assert_eq!(from_enum, KINDS, "KINDS must list every DocxTransitionalMutation variant, in declaration order");

        let manifest = include_str!("../../🧪️oracle/🔣️component.json");
        let needle = "\"kinds\": [";
        let start = manifest.find(needle).expect("manifest declares a kinds array") + needle.len();
        let end = start + manifest[start..].find(']').expect("kinds array is closed");
        let declared: Vec<String> = manifest[start..end].split(',').map(|entry| entry.trim().trim_matches('"').to_string()).filter(|entry| !entry.is_empty()).collect();
        assert_eq!(declared, KINDS, "the oracle manifest's kinds must match DocxTransitionalMutation exactly");
    }
    //#endregion 🔖️KindsConformanceLaw

    //#region 🔖️StampLaw
    /// 🏅️ The class stamp is bijective: stamping into one class and back out of it lands on the
    /// snapshot it started from. This is what makes `SetSnapshot` exactly invertible on this axis,
    /// and it is proven on a snapshot built by this repository's own code, not asserted.
    #[test]
    fn stamping_into_a_class_and_back_is_the_identity() {
        let base = DocxSnapshot::default();
        assert_eq!(stamp_conformance_class(stamp_conformance_class(base.clone(), true), false), stamp_conformance_class(base, false));
    }
    //#endregion 🔖️StampLaw
}
//#endregion 🧪️Tests
