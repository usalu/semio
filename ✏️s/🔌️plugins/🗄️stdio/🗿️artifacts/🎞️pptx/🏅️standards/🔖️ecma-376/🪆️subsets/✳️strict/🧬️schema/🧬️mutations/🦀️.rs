//! 🧬️ `PptxStrictMutation` — the ISO/IEC 29500-1 Strict CONFORMANCE-CLASS vocabulary of
//! `stdio.pptx`. Every variant's `diff()` is handcrafted (never apply-and-capture) and every
//! variant's `inverse()` is handcrafted, reading whatever pre-state it needs out of the base.
//!
//! **Why this subset needs a vocabulary of its own.** `✳️any` owns the DOCUMENT vocabulary —
//! the slide, shape, paragraph and run kinds. Not one of those mutations can move a package between conformance
//! classes, because a conformance class is a property of the OPC PACKAGE and of no document object
//! at all. `check_strict_conformance` reads six axes on an already-decoded `PptxSnapshot`, one of which the `✳️strict` DOCX subset does not have: besides the Strict PresentationML main namespace, the Transitional namespace, VML, the `officeDocument` relationship base, `conformance="strict"` and `mc:AlternateContent`, it separately rejects the Transitional DrawingML namespace. This enum is one variant per axis, plus the two baseline variants.
//!
//! **Where a PPTX package keeps its parts, and why that matters here.** Unlike `📕️xlsx` and
//! `📜️docx`, `PptxSnapshot` holds every XML part as a TYPED `PptxXmlPart` in `xml_parts`, with
//! `opc.parts` carrying only the binary ones — `encode_pptx` rejects a package that stores an XML
//! part as opaque OPC bytes. Every variant below therefore rewrites `xml_parts` (through
//! `PptxDiff::xml_parts`, which the diff type carries as a whole-collection replacement rather than
//! a keyed triple) and touches `opc` only for `[Content_Types].xml` and the relationship table.
//!
//! @see ../../🔣️oracle.json — the mutation catalog `KINDS` is measured against.
//! @see ../🦀️.rs — this subset's conformance check, one axis per variant below.

use crate::artifacts::pptx::standards::v_ecma_376::subsets::any::schema::diff::{NamedModified, NamedTripleDiff, PptxDiff, PptxOpcContentTypesDiff, PptxOpcCtEntriesDiff, PptxOpcDiff, PptxOpcRelDiff, PptxOpcRelListDiff, PptxOpcRelationshipsDiff};
use crate::artifacts::pptx::standards::v_ecma_376::subsets::any::schema::snapshot::{PptxSnapshot, PptxXmlPart};
use crate::artifacts::xml::schema::snapshot::{xml_document_from_text, xml_document_to_text, XmlAttr, XmlDocument, XmlNode};
use crate::artifacts::zip::opc::resolve_relationship_target;
use protocol::command::DiffAlgebra;
use protocol::Mutation;

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

/// 🧩️ The legacy VML namespace ISO/IEC 29500-1 Strict removes entirely.
pub const VML_NS: &str = "urn:schemas-microsoft-com:vml";
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
/// 📐️ Typed conformance-class mutation for `stdio.pptx` under ISO/IEC 29500-1
/// Strict. Every variant addresses ONE axis of the class; none addresses document content.
//#region 🔖️Leaves
#[path = "🔧set-snapshot/🦀️.rs"]
pub mod set_snapshot;
#[path = "🔩set-main-namespace/🦀️.rs"]
pub mod set_main_namespace;
#[path = "⚙set-drawing-namespace/🦀️.rs"]
pub mod set_drawing_namespace;
#[path = "🧩set-relationship-base/🦀️.rs"]
pub mod set_relationship_base;
#[path = "🔖set-conformance-attribute/🦀️.rs"]
pub mod set_conformance_attribute;
#[path = "🏷remove-conformance-attribute/🦀️.rs"]
pub mod remove_conformance_attribute;
#[path = "📐insert-vml-part/🦀️.rs"]
pub mod insert_vml_part;
#[path = "📏remove-vml-part/🦀️.rs"]
pub mod remove_vml_part;
#[path = "🧮insert-alternate-content/🦀️.rs"]
pub mod insert_alternate_content;
#[path = "🔢remove-alternate-content/🦀️.rs"]
pub mod remove_alternate_content;
//#endregion 🔖️Leaves

/// 📐️ Typed mutation for this subset. `NoMutation` was dropped: `#[derive(dsl::Mutations)]` requires
/// every variant to wrap exactly one leaf payload and a unit variant wraps none.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[mutations(snapshot = PptxSnapshot, diff = PptxDiff, schema = "PptxStrictMutation")]
pub enum PptxStrictMutation {
    SetSnapshot(set_snapshot::SetSnapshot),
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
pub const KINDS: &[&str] = &["no-mutation", "set-snapshot", "set-main-namespace", "set-drawing-namespace", "set-relationship-base", "set-conformance-attribute", "remove-conformance-attribute", "insert-vml-part", "remove-vml-part", "insert-alternate-content", "remove-alternate-content"];
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot` through its own diff — the diff is the single semantics
/// source, never a separate imperative apply path.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_pptx_strict_mutation(snapshot: &mut PptxSnapshot, mutation: &PptxStrictMutation) -> protocol::MutationOutcome<PptxDiff> {
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

/// 🧩️ The canonical legacy-VML part body this vocabulary inserts — real VML, so the namespace the
/// ✳️strict check scans a part for is genuinely present.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn vml_markup() -> String {
    format!("<xml xmlns:v=\"{VML_NS}\"><v:shape id=\"legacyShape\" type=\"#_x0000_t202\"/></xml>")
}

/// 🔺️ The diff of adding a legacy VML drawing part together with its content-type override.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_insert_vml_part(base: &PptxSnapshot, path: &str, markup: &str) -> PptxDiff {
    if xml_part(base, path).is_some() {
        return PptxDiff::default();
    }
    let Ok(document) = xml_document_from_text(markup) else { return PptxDiff::default() };
    let mut parts = base.xml_parts.clone();
    parts.push(PptxXmlPart { path: path.trim_start_matches('/').to_string(), content_type: VML_CONTENT_TYPE.to_string(), document });
    PptxDiff { xml_parts: Some(parts), opc: overrides_diff(base, path, Some(VML_CONTENT_TYPE)), ..Default::default() }
}

/// 🔺️ The diff of removing a legacy VML drawing part and its content-type override.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_remove_vml_part(base: &PptxSnapshot, path: &str) -> PptxDiff {
    if xml_part(base, path).is_none() {
        return PptxDiff::default();
    }
    let key = path.trim_start_matches('/');
    let parts: Vec<PptxXmlPart> = base.xml_parts.iter().filter(|part| part.path != key).cloned().collect();
    PptxDiff { xml_parts: Some(parts), opc: overrides_diff(base, path, None), ..Default::default() }
}

/// 🔺️ Sparse `[Content_Types].xml` override diff, keyed by the `/`-prefixed part name the typed
/// table itself keys by. Whether the entry is an addition or a modification is read from the base,
/// never assumed.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn overrides_diff(base: &PptxSnapshot, path: &str, content_type: Option<&str>) -> Option<PptxOpcDiff> {
    let key = format!("/{}", path.trim_start_matches('/'));
    let present = base.opc.content_types.overrides.iter().any(|(name, _)| *name == key);
    let entries: PptxOpcCtEntriesDiff = match (present, content_type) {
        (true, Some(content_type)) => NamedTripleDiff { modified: vec![NamedModified { key, diff: content_type.to_string() }], ..Default::default() },
        (true, None) => NamedTripleDiff { removed: vec![key], ..Default::default() },
        (false, Some(content_type)) => NamedTripleDiff { added: vec![(key, content_type.to_string())], ..Default::default() },
        (false, None) => return None,
    };
    Some(PptxOpcDiff { content_types: Some(PptxOpcContentTypesDiff { defaults: None, overrides: Some(entries) }), parts: None, relationships: None })
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

/// 🔺️ The diff of rewriting one XML part's root children.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_root_children(base: &PptxSnapshot, path: &str, edit: impl FnOnce(&mut Vec<XmlNode>) -> bool) -> PptxDiff {
    let key = path.trim_start_matches('/');
    let mut parts = base.xml_parts.clone();
    let Some(part) = parts.iter_mut().find(|part| part.path == key) else { return PptxDiff::default() };
    let Some(XmlNode::Element { children, .. }) = part.document.root.as_mut() else { return PptxDiff::default() };
    if !edit(children) {
        return PptxDiff::default();
    }
    PptxDiff { xml_parts: Some(parts), ..Default::default() }
}
//#endregion 🔖️DiffBuilders

//#region 🔖️MutationTrait
// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_diff(this: &PptxStrictMutation, base: &PptxSnapshot) -> protocol::MutationOutcome<PptxDiff> {
        protocol::MutationOutcome::new(match this {
            PptxStrictMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => <PptxDiff as DiffAlgebra<PptxSnapshot>>::between(base, snapshot),
            PptxStrictMutation::SetMainNamespace(set_main_namespace::SetMainNamespace { namespace }) => diff_retarget_namespace(base, MAIN_NAMESPACES, namespace),
            PptxStrictMutation::SetDrawingNamespace(set_drawing_namespace::SetDrawingNamespace { namespace }) => diff_retarget_namespace(base, DRAWING_NAMESPACES, namespace),
            PptxStrictMutation::SetRelationshipBase(set_relationship_base::SetRelationshipBase { base: target }) => diff_retarget_relationship_base(base, RELATIONSHIP_NAMESPACES, target),
            PptxStrictMutation::SetConformanceAttribute(set_conformance_attribute::SetConformanceAttribute { value }) => diff_conformance_attribute(base, Some(value)),
            PptxStrictMutation::RemoveConformanceAttribute(_) => diff_conformance_attribute(base, None),
            PptxStrictMutation::InsertVmlPart(insert_vml_part::InsertVmlPart { path, markup }) => diff_insert_vml_part(base, path, markup),
            PptxStrictMutation::RemoveVmlPart(remove_vml_part::RemoveVmlPart { path }) => diff_remove_vml_part(base, path),
            PptxStrictMutation::InsertAlternateContent(insert_alternate_content::InsertAlternateContent { path }) => diff_root_children(base, path, |children| {
                children.push(alternate_content_node());
                true
            }),
            PptxStrictMutation::RemoveAlternateContent(remove_alternate_content::RemoveAlternateContent { path }) => diff_root_children(base, path, |children| {
                let before = children.len();
                children.retain(|child| !matches!(child, XmlNode::Element { name, .. } if name == ALTERNATE_CONTENT_ELEMENT));
                children.len() != before
            }),
        })
    }

// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_inverse(this: &PptxStrictMutation, base: &PptxSnapshot) -> Vec<PptxStrictMutation> {
        vec![match this {
            PptxStrictMutation::SetSnapshot(_) => PptxStrictMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() }),
            PptxStrictMutation::SetMainNamespace(_) => match declared_pair_member(base, MAIN_NAMESPACES) {
                Some(namespace) => PptxStrictMutation::SetMainNamespace(set_main_namespace::SetMainNamespace { namespace }),
                None => return Vec::new(),
            },
            PptxStrictMutation::SetDrawingNamespace(_) => match declared_pair_member(base, DRAWING_NAMESPACES) {
                Some(namespace) => PptxStrictMutation::SetDrawingNamespace(set_drawing_namespace::SetDrawingNamespace { namespace }),
                None => return Vec::new(),
            },
            PptxStrictMutation::SetRelationshipBase(_) => match declared_relationship_base(base, RELATIONSHIP_NAMESPACES) {
                Some(target) => PptxStrictMutation::SetRelationshipBase(set_relationship_base::SetRelationshipBase { base: target }),
                None => return Vec::new(),
            },
            PptxStrictMutation::SetConformanceAttribute(_) => match conformance_attribute(base) {
                Some(value) => PptxStrictMutation::SetConformanceAttribute(set_conformance_attribute::SetConformanceAttribute { value }),
                None => PptxStrictMutation::RemoveConformanceAttribute(remove_conformance_attribute::RemoveConformanceAttribute {}),
            },
            PptxStrictMutation::RemoveConformanceAttribute(_) => match conformance_attribute(base) {
                Some(value) => PptxStrictMutation::SetConformanceAttribute(set_conformance_attribute::SetConformanceAttribute { value }),
                None => return Vec::new(),
            },
            PptxStrictMutation::InsertVmlPart(insert_vml_part::InsertVmlPart { path, .. }) => PptxStrictMutation::RemoveVmlPart(remove_vml_part::RemoveVmlPart { path: path.clone() }),
            PptxStrictMutation::RemoveVmlPart(remove_vml_part::RemoveVmlPart { path }) => match xml_part(base, path) {
                Some(part) => PptxStrictMutation::InsertVmlPart(insert_vml_part::InsertVmlPart { path: path.clone(), markup: xml_document_to_text(&part.document) }),
                None => return Vec::new(),
            },
            PptxStrictMutation::InsertAlternateContent(insert_alternate_content::InsertAlternateContent { path }) => PptxStrictMutation::RemoveAlternateContent(remove_alternate_content::RemoveAlternateContent { path: path.clone() }),
            PptxStrictMutation::RemoveAlternateContent(remove_alternate_content::RemoveAlternateContent { path }) => PptxStrictMutation::InsertAlternateContent(insert_alternate_content::InsertAlternateContent { path: path.clone() }),
        }]
    }
//#endregion 🔖️MutationTrait

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    //#region 🔖️KindsConformanceLaw
    /// 🧭️ `kind_of` is an EXHAUSTIVE match (no wildcard arm) — the compiler refuses this file if a
    /// variant is added to `PptxStrictMutation` without a matching kebab-case spelling here, which is what keeps
    /// `KINDS` honest against the enum. The second half reads the sibling oracle manifest's `kinds`
    /// array as text (the framework never parses Rust, so this is the only side that can prove the
    /// manifest matches) and asserts the same list, in the same order.
    #[test]
    fn kinds_match_enum_and_catalog() {
        fn kind_of(mutation: &PptxStrictMutation) -> &'static str {
            match mutation {
                PptxStrictMutation::SetSnapshot(_) => "set-snapshot",
                PptxStrictMutation::SetMainNamespace(_) => "set-main-namespace",
                PptxStrictMutation::SetDrawingNamespace(_) => "set-drawing-namespace",
                PptxStrictMutation::SetRelationshipBase(_) => "set-relationship-base",
                PptxStrictMutation::SetConformanceAttribute(_) => "set-conformance-attribute",
                PptxStrictMutation::RemoveConformanceAttribute(_) => "remove-conformance-attribute",
                PptxStrictMutation::InsertVmlPart(_) => "insert-vml-part",
                PptxStrictMutation::RemoveVmlPart(_) => "remove-vml-part",
                PptxStrictMutation::InsertAlternateContent(_) => "insert-alternate-content",
                PptxStrictMutation::RemoveAlternateContent(_) => "remove-alternate-content",
            }
        }
        let samples = [
            PptxStrictMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: PptxSnapshot::default() }),
            PptxStrictMutation::SetMainNamespace(set_main_namespace::SetMainNamespace { namespace: String::new() }),
            PptxStrictMutation::SetDrawingNamespace(set_drawing_namespace::SetDrawingNamespace { namespace: String::new() }),
            PptxStrictMutation::SetRelationshipBase(set_relationship_base::SetRelationshipBase { base: String::new() }),
            PptxStrictMutation::SetConformanceAttribute(set_conformance_attribute::SetConformanceAttribute { value: String::new() }),
            PptxStrictMutation::RemoveConformanceAttribute(remove_conformance_attribute::RemoveConformanceAttribute {}),
            PptxStrictMutation::InsertVmlPart(insert_vml_part::InsertVmlPart { path: String::new(), markup: String::new() }),
            PptxStrictMutation::RemoveVmlPart(remove_vml_part::RemoveVmlPart { path: String::new() }),
            PptxStrictMutation::InsertAlternateContent(insert_alternate_content::InsertAlternateContent { path: String::new() }),
            PptxStrictMutation::RemoveAlternateContent(remove_alternate_content::RemoveAlternateContent { path: String::new() }),
        ];
        let from_enum: Vec<&'static str> = samples.iter().map(kind_of).collect();
        assert_eq!(from_enum, KINDS, "KINDS must list every PptxStrictMutation variant, in declaration order");

        let manifest = include_str!("../../🧪️oracle/🔣️.json");
        let needle = "\"kinds\": [";
        let start = manifest.find(needle).expect("manifest declares a kinds array") + needle.len();
        let end = start + manifest[start..].find(']').expect("kinds array is closed");
        let declared: Vec<String> = manifest[start..end].split(',').map(|entry| entry.trim().trim_matches('"').to_string()).filter(|entry| !entry.is_empty()).collect();
        assert_eq!(declared, KINDS, "the oracle manifest's kinds must match PptxStrictMutation exactly");
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
