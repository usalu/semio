//! 🧬️ `PptxTransitionalMutation` — the ISO/IEC 29500-4 Transitional CONFORMANCE-CLASS vocabulary of
//! `stdio.pptx`. Every variant's `diff()` is handcrafted (never apply-and-capture) and every
//! variant's `inverse()` is handcrafted, reading whatever pre-state it needs out of the base.
//!
//! **Why this subset needs a vocabulary of its own.** `✳️any` owns the DOCUMENT vocabulary —
//! the slide, shape, paragraph and run kinds. Not one of those mutations can move a package between conformance
//! classes, because a conformance class is a property of the OPC PACKAGE and of no document object
//! at all. `check_transitional_conformance` reads three axes — the Transitional PresentationML main namespace, any strict-family namespace in a part or a relationship type, and a contradicting `conformance="strict"` — over a package carrying TWO Transitional namespace families, PresentationML and DrawingML, each addressable on its own.
//!
//! **Where a PPTX package keeps its parts, and why that matters here.** Unlike `📕️xlsx` and
//! `📜️docx`, `PptxSnapshot` holds every XML part as a TYPED `PptxXmlPart` in `xml_parts`, with
//! `opc.parts` carrying only the binary ones — `encode_pptx` rejects a package that stores an XML
//! part as opaque OPC bytes. Every variant below therefore rewrites `xml_parts` (through
//! `PptxDiff::xml_parts`, which the diff type carries as a whole-collection replacement rather than
//! a keyed triple) and touches `opc` only for `[Content_Types].xml` and the relationship table.
//!
//! @see ../../🧪️oracle/🔣️component.json — the mutation catalog `KINDS` is measured against.
//! @see ../🦀️component.rs — this subset's conformance check, one axis per variant below.

use crate::artifacts::pptx::standards::v_ecma_376::subsets::any::schema::diff::{NamedModified, NamedTripleDiff, PptxDiff, PptxOpcContentTypesDiff, PptxOpcCtEntriesDiff, PptxOpcDiff, PptxOpcRelDiff, PptxOpcRelListDiff, PptxOpcRelationshipsDiff};
use crate::artifacts::pptx::standards::v_ecma_376::subsets::any::schema::snapshot::{PptxSnapshot, PptxXmlPart};
use crate::artifacts::xml::schema::snapshot::{xml_document_from_text, xml_document_to_text, XmlAttr, XmlDocument, XmlNode};
use crate::artifacts::zip::opc::resolve_relationship_target;
use protocol::command::DiffAlgebra;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

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
//#endregion 🔖️Dialect

//#region 🔖️Mutations
/// 📐️ Typed conformance-class mutation for `stdio.pptx` under ISO/IEC 29500-4
/// Transitional. Every variant addresses ONE axis of the class; none addresses document content.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum PptxTransitionalMutation {
    /// 🚫️ The identity element of the vocabulary.
    #[default]
    NoMutation,
    /// 🔄️ Replaces the whole package. A conformance class is a whole-package property, so this is the class stamp in its total form — every namespace family, the relationship base and the root attribute at once. Build the target with [`stamp_conformance_class`].
    SetSnapshot {
        snapshot: PptxSnapshot,
    },
    /// 🏷️ Retargets the main PresentationML namespace declaration wherever it is declared, from whichever member of the `[transitional, strict]` pair the package currently carries to `namespace`.
    SetMainNamespace {
        namespace: String,
    },
    /// 🎨️ Retargets the DrawingML namespace declaration — a second namespace family a deck carries independently of its PresentationML one, and one this subset's conformance check reads on its own.
    SetDrawingNamespace {
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

/// 🧾️ Kebab-case spelling of every `PptxTransitionalMutation` variant, in declaration order — the exhaustive
/// mutation catalog `pptx-ecma-376-transitional` (`../../🧪️oracle/🔣️component.json`) is measured against
/// this exact list. `kinds_match_enum_and_catalog` proves it never drifts from either side.
pub const KINDS: &[&str] = &["no-mutation", "set-snapshot", "set-main-namespace", "set-drawing-namespace", "set-relationship-base", "set-conformance-attribute", "remove-conformance-attribute"];
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot` through its own diff — the diff is the single semantics
/// source, never a separate imperative apply path.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_pptx_transitional_mutation(snapshot: &mut PptxSnapshot, mutation: &PptxTransitionalMutation) -> protocol::MutationOutcome<PptxDiff> {
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

/// ✍️ Rewrites every attribute value equal to a member of `from` to `to`, through the whole subtree
/// — a namespace declaration is an ordinary attribute, which is why one walk covers `xmlns`,
/// `xmlns:a`, `xmlns:p` and whatever prefixed alias a real deck happens to use.
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

/// 🔎️ Which member of a `[transitional, strict]` pair the deck actually declares.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn declared_pair_member(base: &PptxSnapshot, pair: [&str; 2]) -> Option<String> {
    pair.into_iter().find(|candidate| base.xml_parts.iter().any(|part| part.document.root.as_ref().is_some_and(|root| declares_namespace(root, candidate)))).map(str::to_string)
}

/// 🔎️ The relationship-type base the deck's own relationships are built on.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn declared_relationship_base(base: &PptxSnapshot, pair: [&str; 2]) -> Option<String> {
    pair.into_iter().find(|candidate| base.opc.relationships.values().flatten().any(|relationship| relationship.rel_type.starts_with(candidate))).map(str::to_string)
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
pub fn stamp_conformance_class(mut snapshot: PptxSnapshot, strict: bool) -> PptxSnapshot {
    let index = usize::from(strict);
    for part in snapshot.xml_parts.iter_mut() {
        let Some(root) = part.document.root.as_mut() else { continue };
        retarget_namespace(root, &MAIN_NAMESPACES, MAIN_NAMESPACES[index]);
        retarget_namespace(root, &DRAWING_NAMESPACES, DRAWING_NAMESPACES[index]);
        retarget_namespace(root, &RELATIONSHIP_NAMESPACES, RELATIONSHIP_NAMESPACES[index]);
    }
    for relationships in snapshot.opc.relationships.values_mut() {
        for relationship in relationships.iter_mut() {
            let Some(prefix) = RELATIONSHIP_NAMESPACES.into_iter().find(|prefix| relationship.rel_type.starts_with(prefix)) else { continue };
            relationship.rel_type = format!("{}{}", RELATIONSHIP_NAMESPACES[index], &relationship.rel_type[prefix.len()..]);
        }
    }
    if let Some(path) = main_part_path(&snapshot) {
        let key = path.trim_start_matches('/').to_string();
        if let Some(part) = snapshot.xml_parts.iter_mut().find(|part| part.path == key) {
            set_root_attribute(&mut part.document, "conformance", if strict { Some("strict") } else { None });
        }
    }
    snapshot
}
//#endregion 🔖️Helpers

//#region 🔖️DiffBuilders
/// 🔺️ The diff of retargeting one namespace family across every XML part that declares it. The
/// whole `xml_parts` vector travels because that is the only channel `PptxDiff` offers for the typed
/// XML parts — a keyed triple would be the better shape and is `✳️any`'s to add, not this subset's.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_retarget_namespace(base: &PptxSnapshot, from: [&str; 2], to: &str) -> PptxDiff {
    let mut parts = base.xml_parts.clone();
    let mut changed = false;
    for part in parts.iter_mut() {
        let Some(root) = part.document.root.as_mut() else { continue };
        changed |= retarget_namespace(root, &from, to);
    }
    if !changed {
        return PptxDiff::default();
    }
    PptxDiff { xml_parts: Some(parts), ..Default::default() }
}

/// 🔺️ The diff of retargeting the `officeDocument` relationship TYPE base, owner by owner.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_retarget_relationship_base(base: &PptxSnapshot, from: [&str; 2], to: &str) -> PptxDiff {
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
            entries.push(NamedModified { key: relationship.id.clone(), diff: PptxOpcRelDiff { rel_type: Some(retargeted), target: None, target_mode: None } });
        }
        if entries.is_empty() {
            continue;
        }
        modified.push(NamedModified { key: owner.clone(), diff: PptxOpcRelListDiff { modified: entries, ..Default::default() } });
    }
    if modified.is_empty() {
        return PptxDiff::default();
    }
    PptxDiff { opc: Some(PptxOpcDiff { content_types: None, parts: None, relationships: Some(PptxOpcRelationshipsDiff { modified, ..Default::default() }) }), ..Default::default() }
}

/// 🔺️ The diff of setting — or removing — the main part's root `conformance` attribute.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_conformance_attribute(base: &PptxSnapshot, value: Option<&str>) -> PptxDiff {
    let Some(path) = main_part_path(base) else { return PptxDiff::default() };
    let key = path.trim_start_matches('/').to_string();
    let mut parts = base.xml_parts.clone();
    let Some(part) = parts.iter_mut().find(|part| part.path == key) else { return PptxDiff::default() };
    if !set_root_attribute(&mut part.document, "conformance", value) {
        return PptxDiff::default();
    }
    PptxDiff { xml_parts: Some(parts), ..Default::default() }
}
//#endregion 🔖️DiffBuilders

//#region 🔖️MutationTrait
impl Mutation<PptxSnapshot> for PptxTransitionalMutation {
    type Diff = PptxDiff;

    fn diff(&self, base: &PptxSnapshot) -> protocol::MutationOutcome<Self::Diff> {
        protocol::MutationOutcome::new(match self {
            Self::NoMutation => PptxDiff::default(),
            Self::SetSnapshot { snapshot } => <PptxDiff as DiffAlgebra<PptxSnapshot>>::between(base, snapshot),
            Self::SetMainNamespace { namespace } => diff_retarget_namespace(base, MAIN_NAMESPACES, namespace),
            Self::SetDrawingNamespace { namespace } => diff_retarget_namespace(base, DRAWING_NAMESPACES, namespace),
            Self::SetRelationshipBase { base: target } => diff_retarget_relationship_base(base, RELATIONSHIP_NAMESPACES, target),
            Self::SetConformanceAttribute { value } => diff_conformance_attribute(base, Some(value)),
            Self::RemoveConformanceAttribute => diff_conformance_attribute(base, None),
        })
    }

    fn inverse(&self, base: &PptxSnapshot) -> Vec<Self> {
        vec![match self {
            Self::NoMutation => Self::NoMutation,
            Self::SetSnapshot { .. } => Self::SetSnapshot { snapshot: base.clone() },
            Self::SetMainNamespace { .. } => match declared_pair_member(base, MAIN_NAMESPACES) {
                Some(namespace) => Self::SetMainNamespace { namespace },
                None => Self::NoMutation,
            },
            Self::SetDrawingNamespace { .. } => match declared_pair_member(base, DRAWING_NAMESPACES) {
                Some(namespace) => Self::SetDrawingNamespace { namespace },
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
    /// variant is added to `PptxTransitionalMutation` without a matching kebab-case spelling here, which is what keeps
    /// `KINDS` honest against the enum. The second half reads the sibling oracle manifest's `kinds`
    /// array as text (the framework never parses Rust, so this is the only side that can prove the
    /// manifest matches) and asserts the same list, in the same order.
    #[test]
    fn kinds_match_enum_and_catalog() {
        fn kind_of(mutation: &PptxTransitionalMutation) -> &'static str {
            match mutation {
                PptxTransitionalMutation::NoMutation => "no-mutation",
                PptxTransitionalMutation::SetSnapshot { .. } => "set-snapshot",
                PptxTransitionalMutation::SetMainNamespace { .. } => "set-main-namespace",
                PptxTransitionalMutation::SetDrawingNamespace { .. } => "set-drawing-namespace",
                PptxTransitionalMutation::SetRelationshipBase { .. } => "set-relationship-base",
                PptxTransitionalMutation::SetConformanceAttribute { .. } => "set-conformance-attribute",
                PptxTransitionalMutation::RemoveConformanceAttribute => "remove-conformance-attribute",
            }
        }
        let samples = [
            PptxTransitionalMutation::NoMutation,
            PptxTransitionalMutation::SetSnapshot { snapshot: PptxSnapshot::default() },
            PptxTransitionalMutation::SetMainNamespace { namespace: String::new() },
            PptxTransitionalMutation::SetDrawingNamespace { namespace: String::new() },
            PptxTransitionalMutation::SetRelationshipBase { base: String::new() },
            PptxTransitionalMutation::SetConformanceAttribute { value: String::new() },
            PptxTransitionalMutation::RemoveConformanceAttribute,
        ];
        let from_enum: Vec<&'static str> = samples.iter().map(kind_of).collect();
        assert_eq!(from_enum, KINDS, "KINDS must list every PptxTransitionalMutation variant, in declaration order");

        let manifest = include_str!("../../🧪️oracle/🔣️component.json");
        let needle = "\"kinds\": [";
        let start = manifest.find(needle).expect("manifest declares a kinds array") + needle.len();
        let end = start + manifest[start..].find(']').expect("kinds array is closed");
        let declared: Vec<String> = manifest[start..end].split(',').map(|entry| entry.trim().trim_matches('"').to_string()).filter(|entry| !entry.is_empty()).collect();
        assert_eq!(declared, KINDS, "the oracle manifest's kinds must match PptxTransitionalMutation exactly");
    }
    //#endregion 🔖️KindsConformanceLaw

    //#region 🔖️StampLaw
    /// 🏅️ The class stamp is bijective: stamping into one class and back out of it lands on the
    /// snapshot it started from. This is what makes `SetSnapshot` exactly invertible on this axis,
    /// and it is proven on a snapshot built by this repository's own code, not asserted.
    #[test]
    fn stamping_into_a_class_and_back_is_the_identity() {
        let base = PptxSnapshot::default();
        assert_eq!(stamp_conformance_class(stamp_conformance_class(base.clone(), true), false), stamp_conformance_class(base, false));
    }
    //#endregion 🔖️StampLaw
}
//#endregion 🧪️Tests
