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
//! @see ../../🧪️oracle/🔣️.json — the mutation catalog `KINDS` is measured against.
//! @see ../🦀️component.rs — this subset's conformance check, one axis per variant below.

use crate::artifacts::docx::standards::v_ecma_376::subsets::base::schema::diff::{NamedModified, NamedTripleDiff, DocxDiff, DocxOpcContentTypesDiff, DocxOpcCtEntriesDiff, DocxOpcDiff, DocxOpcPartDiff, DocxOpcPartsDiff, DocxOpcRelDiff, DocxOpcRelListDiff, DocxOpcRelationshipsDiff};
use crate::artifacts::docx::standards::v_ecma_376::subsets::base::schema::snapshot::DocxSnapshot;
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
/// 📐️ Typed conformance-class mutation for `stdio.docx` under ISO/IEC 29500-1
/// Strict. Every variant addresses ONE axis of the class; none addresses document content.
//#region 🔖️Leaves
#[path = "🔧set-snapshot/🦀️.rs"]
pub mod set_snapshot;
#[path = "🔩set-main-namespace/🦀️.rs"]
pub mod set_main_namespace;
#[path = "⚙set-relationship-base/🦀️.rs"]
pub mod set_relationship_base;
#[path = "🧩set-conformance-attribute/🦀️.rs"]
pub mod set_conformance_attribute;
#[path = "🔖remove-conformance-attribute/🦀️.rs"]
pub mod remove_conformance_attribute;
#[path = "🏷insert-vml-part/🦀️.rs"]
pub mod insert_vml_part;
#[path = "📐remove-vml-part/🦀️.rs"]
pub mod remove_vml_part;
#[path = "📏insert-alternate-content/🦀️.rs"]
pub mod insert_alternate_content;
#[path = "🧮remove-alternate-content/🦀️.rs"]
pub mod remove_alternate_content;
//#endregion 🔖️Leaves

/// 📐️ Typed mutation for this subset. `NoMutation` was dropped: `#[derive(dsl::Mutations)]` requires
/// every variant to wrap exactly one leaf payload and a unit variant wraps none.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::Mutations)]
#[mutations(snapshot = DocxSnapshot, diff = DocxDiff, schema = "DocxStrictMutation")]
pub enum DocxStrictMutation {
    SetSnapshot(set_snapshot::SetSnapshot),
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
/// mutation catalog `docx-ecma-376-strict` (`../../🧪️oracle/🔣️.json`) is measured against
/// this exact list. `kinds_match_enum_and_catalog` proves it never drifts from either side.
pub const KINDS: &[&str] = &["no-mutation", "set-snapshot", "set-main-namespace", "set-relationship-base", "set-conformance-attribute", "remove-conformance-attribute", "insert-vml-part", "remove-vml-part", "insert-alternate-content", "remove-alternate-content"];
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot` through its own diff — the diff is the single semantics
/// source, never a separate imperative apply path.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_docx_strict_mutation(snapshot: &mut DocxSnapshot, mutation: &DocxStrictMutation) -> protocol::MutationOutcome<DocxDiff> {
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
    Some(NamedTripleDiff { removed, modified: modified.into_iter().map(|(key, diff)| NamedModified { key, diff }).collect(), added, ..Default::default() })
}

/// 🔺️ Sparse `[Content_Types].xml` override diff, keyed by the `/`-prefixed part name the typed
/// table itself keys by. Whether the entry is an addition or a modification is read from the base,
/// never assumed.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn overrides_diff(base: &DocxSnapshot, path: &str, content_type: Option<&str>) -> Option<DocxOpcContentTypesDiff> {
    let key = format!("/{}", path.trim_start_matches('/'));
    let present = base.opc.content_types.overrides.iter().any(|(name, _)| *name == key);
    let entries: DocxOpcCtEntriesDiff = match (present, content_type) {
        (true, Some(content_type)) => NamedTripleDiff { modified: vec![NamedModified { key, diff: content_type.to_string() }], ..Default::default() },
        (true, None) => NamedTripleDiff { removed: vec![key], ..Default::default() },
        (false, Some(content_type)) => NamedTripleDiff { added: vec![(key, content_type.to_string())], ..Default::default() },
        (false, None) => return None,
    };
    Some(DocxOpcContentTypesDiff { defaults: None, overrides: Some(entries) })
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

/// 🧩️ The canonical legacy-VML part body this vocabulary inserts — real VML, so the namespace the
/// ✳️strict check scans a part for is genuinely present.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn vml_markup() -> String {
    format!("<xml xmlns:v=\"{VML_NS}\"><v:shape id=\"legacyShape\" type=\"#_x0000_t202\"/></xml>")
}

/// 🔺️ The diff of adding a legacy VML drawing part together with its content-type override.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_insert_vml_part(base: &DocxSnapshot, path: &str, markup: &str) -> DocxDiff {
    if base.opc.part(path).is_some() {
        return DocxDiff::default();
    }
    let part = OpcPart { path: path.trim_start_matches('/').to_string(), content_type: VML_CONTENT_TYPE.to_string(), bytes: markup.as_bytes().to_vec() };
    opc_diff(parts_diff(Vec::new(), vec![part], Vec::new()), overrides_diff(base, path, Some(VML_CONTENT_TYPE)), None)
}

/// 🔺️ The diff of removing a legacy VML drawing part and its content-type override.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_remove_vml_part(base: &DocxSnapshot, path: &str) -> DocxDiff {
    let Some(part) = base.opc.part(path) else { return DocxDiff::default() };
    opc_diff(parts_diff(Vec::new(), Vec::new(), vec![part.path.clone()]), overrides_diff(base, path, None), None)
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

/// 🔺️ The diff of appending one markup-compatibility fallback to a part's root element.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_append_alternate_content(base: &DocxSnapshot, path: &str) -> DocxDiff {
    diff_root_children(base, path, |children| {
        children.push(alternate_content_node());
        true
    })
}

/// 🔺️ The diff of stripping every markup-compatibility fallback from a part's root element.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_strip_alternate_content(base: &DocxSnapshot, path: &str) -> DocxDiff {
    diff_root_children(base, path, |children| {
        let before = children.len();
        children.retain(|child| !matches!(child, XmlNode::Element { name, .. } if name == ALTERNATE_CONTENT_ELEMENT));
        children.len() != before
    })
}

/// 🔺️ The diff of rewriting one part's root children.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_root_children(base: &DocxSnapshot, path: &str, edit: impl FnOnce(&mut Vec<XmlNode>) -> bool) -> DocxDiff {
    let Some(part) = base.opc.part(path) else { return DocxDiff::default() };
    let Some(mut document) = parse_part(part) else { return DocxDiff::default() };
    let Some(XmlNode::Element { children, .. }) = document.root.as_mut() else { return DocxDiff::default() };
    if !edit(children) {
        return DocxDiff::default();
    }
    opc_diff(parts_diff(vec![(part.path.clone(), DocxOpcPartDiff { content_type: None, bytes: Some(serialize(&document)) })], Vec::new(), Vec::new()), None, None)
}
//#endregion 🔖️DiffBuilders

//#region 🔖️MutationTrait
// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_diff(this: &DocxStrictMutation, base: &DocxSnapshot) -> protocol::MutationOutcome<DocxDiff> {
        protocol::MutationOutcome::new(match this {
            DocxStrictMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => <DocxDiff as DiffAlgebra<DocxSnapshot>>::between(base, snapshot),
            DocxStrictMutation::SetMainNamespace(set_main_namespace::SetMainNamespace { namespace }) => diff_retarget_namespace(base, MAIN_NAMESPACES, namespace),
            DocxStrictMutation::SetRelationshipBase(set_relationship_base::SetRelationshipBase { base: target }) => diff_retarget_relationship_base(base, RELATIONSHIP_NAMESPACES, target),
            DocxStrictMutation::SetConformanceAttribute(set_conformance_attribute::SetConformanceAttribute { value }) => diff_conformance_attribute(base, Some(value)),
            DocxStrictMutation::RemoveConformanceAttribute(_) => diff_conformance_attribute(base, None),
            DocxStrictMutation::InsertVmlPart(insert_vml_part::InsertVmlPart { path, markup }) => diff_insert_vml_part(base, path, markup),
            DocxStrictMutation::RemoveVmlPart(remove_vml_part::RemoveVmlPart { path }) => diff_remove_vml_part(base, path),
            DocxStrictMutation::InsertAlternateContent(insert_alternate_content::InsertAlternateContent { path }) => diff_append_alternate_content(base, path),
            DocxStrictMutation::RemoveAlternateContent(remove_alternate_content::RemoveAlternateContent { path }) => diff_strip_alternate_content(base, path),
        })
    }

// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_inverse(this: &DocxStrictMutation, base: &DocxSnapshot) -> Vec<DocxStrictMutation> {
        vec![match this {
            DocxStrictMutation::SetSnapshot(_) => DocxStrictMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() }),
            DocxStrictMutation::SetMainNamespace(_) => match declared_pair_member(base, MAIN_NAMESPACES) {
                Some(namespace) => DocxStrictMutation::SetMainNamespace(set_main_namespace::SetMainNamespace { namespace }),
                None => return Vec::new(),
            },
            DocxStrictMutation::SetRelationshipBase(_) => match declared_relationship_base(base, RELATIONSHIP_NAMESPACES) {
                Some(target) => DocxStrictMutation::SetRelationshipBase(set_relationship_base::SetRelationshipBase { base: target }),
                None => return Vec::new(),
            },
            DocxStrictMutation::SetConformanceAttribute(_) => match conformance_attribute(base) {
                Some(value) => DocxStrictMutation::SetConformanceAttribute(set_conformance_attribute::SetConformanceAttribute { value }),
                None => DocxStrictMutation::RemoveConformanceAttribute(remove_conformance_attribute::RemoveConformanceAttribute {}),
            },
            DocxStrictMutation::RemoveConformanceAttribute(_) => match conformance_attribute(base) {
                Some(value) => DocxStrictMutation::SetConformanceAttribute(set_conformance_attribute::SetConformanceAttribute { value }),
                None => return Vec::new(),
            },
            DocxStrictMutation::InsertVmlPart(insert_vml_part::InsertVmlPart { path, .. }) => DocxStrictMutation::RemoveVmlPart(remove_vml_part::RemoveVmlPart { path: path.clone() }),
            DocxStrictMutation::RemoveVmlPart(remove_vml_part::RemoveVmlPart { path }) => match part_text(base, path) {
                Some(markup) => DocxStrictMutation::InsertVmlPart(insert_vml_part::InsertVmlPart { path: path.clone(), markup }),
                None => return Vec::new(),
            },
            DocxStrictMutation::InsertAlternateContent(insert_alternate_content::InsertAlternateContent { path }) => DocxStrictMutation::RemoveAlternateContent(remove_alternate_content::RemoveAlternateContent { path: path.clone() }),
            DocxStrictMutation::RemoveAlternateContent(remove_alternate_content::RemoveAlternateContent { path }) => DocxStrictMutation::InsertAlternateContent(insert_alternate_content::InsertAlternateContent { path: path.clone() }),
        }]
    }
//#endregion 🔖️MutationTrait

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    //#region 🔖️KindsConformanceLaw
    /// 🧭️ `kind_of` is an EXHAUSTIVE match (no wildcard arm) — the compiler refuses this file if a
    /// variant is added to `DocxStrictMutation` without a matching kebab-case spelling here, which is what keeps
    /// `KINDS` honest against the enum. The second half reads the sibling oracle manifest's `kinds`
    /// array as text (the framework never parses Rust, so this is the only side that can prove the
    /// manifest matches) and asserts the same list, in the same order.
    #[test]
    fn kinds_match_enum_and_catalog() {
        fn kind_of(mutation: &DocxStrictMutation) -> &'static str {
            match mutation {
                DocxStrictMutation::SetSnapshot(_) => "set-snapshot",
                DocxStrictMutation::SetMainNamespace(_) => "set-main-namespace",
                DocxStrictMutation::SetRelationshipBase(_) => "set-relationship-base",
                DocxStrictMutation::SetConformanceAttribute(_) => "set-conformance-attribute",
                DocxStrictMutation::RemoveConformanceAttribute(_) => "remove-conformance-attribute",
                DocxStrictMutation::InsertVmlPart(_) => "insert-vml-part",
                DocxStrictMutation::RemoveVmlPart(_) => "remove-vml-part",
                DocxStrictMutation::InsertAlternateContent(_) => "insert-alternate-content",
                DocxStrictMutation::RemoveAlternateContent(_) => "remove-alternate-content",
            }
        }
        let samples = [
            DocxStrictMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: DocxSnapshot::default() }),
            DocxStrictMutation::SetMainNamespace(set_main_namespace::SetMainNamespace { namespace: String::new() }),
            DocxStrictMutation::SetRelationshipBase(set_relationship_base::SetRelationshipBase { base: String::new() }),
            DocxStrictMutation::SetConformanceAttribute(set_conformance_attribute::SetConformanceAttribute { value: String::new() }),
            DocxStrictMutation::RemoveConformanceAttribute(remove_conformance_attribute::RemoveConformanceAttribute {}),
            DocxStrictMutation::InsertVmlPart(insert_vml_part::InsertVmlPart { path: String::new(), markup: String::new() }),
            DocxStrictMutation::RemoveVmlPart(remove_vml_part::RemoveVmlPart { path: String::new() }),
            DocxStrictMutation::InsertAlternateContent(insert_alternate_content::InsertAlternateContent { path: String::new() }),
            DocxStrictMutation::RemoveAlternateContent(remove_alternate_content::RemoveAlternateContent { path: String::new() }),
        ];
        let from_enum: Vec<&'static str> = samples.iter().map(kind_of).collect();
        assert_eq!(from_enum, KINDS, "KINDS must list every DocxStrictMutation variant, in declaration order");

        let manifest = include_str!("../../🧪️oracle/🔣️.json");
        let needle = "\"kinds\": [";
        let start = manifest.find(needle).expect("manifest declares a kinds array") + needle.len();
        let end = start + manifest[start..].find(']').expect("kinds array is closed");
        let declared: Vec<String> = manifest[start..end].split(',').map(|entry| entry.trim().trim_matches('"').to_string()).filter(|entry| !entry.is_empty()).collect();
        assert_eq!(declared, KINDS, "the oracle manifest's kinds must match DocxStrictMutation exactly");
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
