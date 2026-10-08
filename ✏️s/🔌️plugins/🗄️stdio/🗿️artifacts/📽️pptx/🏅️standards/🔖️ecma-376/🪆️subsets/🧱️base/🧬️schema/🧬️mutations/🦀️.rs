//! 🧬️ PptxMutation — document mutation dispatch. Every variant's `diff()` is handcrafted (never
//! apply-and-capture) and every variant's `inverse()` is handcrafted, index-aware.

use crate::schema::diff::PptxDiff;
use crate::schema::snapshot::{PptxParagraph, PptxShape, PptxSlide, PptxTransform};
use crate::PptxSnapshot;
use protocol::Mutation;
use semio_s_artifact_stdio_xml::schema::snapshot::XmlNode;

//#region 🔖️Mutations
#[path = "🔷insert-shape/🦀️.rs"]
pub mod insert_shape;
#[path = "➕insert-slide/🦀️.rs"]
pub mod insert_slide;
#[path = "🔀move-slide/🦀️.rs"]
pub mod move_slide;
#[path = "🔶remove-shape/🦀️.rs"]
pub mod remove_shape;
#[path = "➖remove-slide/🦀️.rs"]
pub mod remove_slide;
#[path = "📐set-shape-position/🦀️.rs"]
pub mod set_shape_position;
#[path = "✍️set-shape-text/🦀️.rs"]
pub mod set_shape_text;
#[path = "🧭️xml-address/🦀️.rs"]
pub mod xml_address;
pub use xml_address::{PptxShapeAddress, PptxSlideAddress, PptxXmlAddress, PptxXmlVacancyAddress};
/// 📐️ Typed content mutation for `stdio.pptx`. Addresses `presentation.slides` by index
/// (slide order matters -- see `MoveSlide`) and, within a slide, `shapes` by
/// `(slide_index, shape_index)` -- a flat two-level address is sufficient since PresentationML
/// slides don't nest arbitrarily (grouped shapes fall back to `PptxShape::Other`, see the
/// snapshot module's doc comment).
//#region 🔖️Leaves
#[path = "🧩️replace-xml-node/🦀️.rs"]
pub mod replace_xml_node;
//#endregion 🔖️Leaves

/// 📐️ Typed mutation for this subset. `NoMutation` was dropped: `#[derive(dsl::Mutations)]` requires
/// every variant to wrap exactly one leaf payload and a unit variant wraps none.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[mutations(snapshot = PptxSnapshot, diff = PptxDiff, schema = "PptxMutation")]
#[value(tag = "mutation", rename_all = "camelCase")]
pub enum PptxMutation {
    /// ➕️ Inserts `🎞️slide` at `index` (FINAL state).
    InsertSlide(insert_slide::InsertSlide),
    /// ➖️ Removes the slide at `index` (BASE-state index).
    RemoveSlide(remove_slide::RemoveSlide),
    /// 🔀️ Moves the slide at BASE-state index `from` to FINAL-state index `to`.
    MoveSlide(move_slide::MoveSlide),
    /// ➕️ Inserts `shape` at `shape_index` (FINAL state) on the slide at `slide_index`.
    InsertShape(insert_shape::InsertShape),
    /// ➖️ Removes the shape at `shape_index` (BASE-state index) on the slide at `slide_index`.
    RemoveShape(remove_shape::RemoveShape),
    /// ✍️ Replaces a `TextBox`/`Placeholder` shape's `text_frame` (no-op on `Picture`/`Other`).
    SetShapeText(set_shape_text::SetShapeText),
    /// 📐️ Sets a shape's `position` (no-op on `Other`, which has none).
    SetShapePosition(set_shape_position::SetShapePosition),
    /// ↩️ Replaces one canonical XML node exactly; the exact inverse of the node-level edits.
    ReplaceXmlNode(replace_xml_node::ReplaceXmlNode),
}
//#endregion 🔖️Mutations

//#region 🔖️Kinds
/// 🏷️ The kebab-case spelling of every `PptxMutation` variant, in the same order the enum
/// declares them — this repository's mutation oracle registrations never parse this enum;
/// `kinds_matches_enum_variants_and_manifest` below is what keeps the two declarations honest
/// against each other.
pub const KINDS: &[&str] = &["insert-slide", "remove-slide", "move-slide", "insert-shape", "remove-shape", "set-shape-text", "set-shape-position", "replace-xml-node"];

/// 🏷️ The `KINDS` spelling of one mutation's own variant. An exhaustive match (no wildcard arm),
/// so a new variant that forgets its kebab spelling here fails to compile rather than failing
/// silently.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn kind_of(mutation: &PptxMutation) -> &'static str {
    match mutation {
        PptxMutation::InsertSlide(_) => "insert-slide",
        PptxMutation::RemoveSlide(_) => "remove-slide",
        PptxMutation::MoveSlide(_) => "move-slide",
        PptxMutation::InsertShape(_) => "insert-shape",
        PptxMutation::RemoveShape(_) => "remove-shape",
        PptxMutation::SetShapeText(_) => "set-shape-text",
        PptxMutation::SetShapePosition(_) => "set-shape-position",
        PptxMutation::ReplaceXmlNode(_) => "replace-xml-node",
    }
}
//#endregion 🔖️Kinds

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`: `let d = mutation.diff(&*snapshot); *snapshot =
/// d.apply(snapshot); d` -- the diff is the single semantics source, never a separate imperative
/// apply path (apply-and-capture is banned).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_pptx_mutation(snapshot: &mut PptxSnapshot, mutation: &PptxMutation) -> protocol::MutationOutcome<PptxDiff> {
    let outcome = Mutation::diff(mutation, snapshot);
    match protocol::apply_diff(outcome.diff(), snapshot) {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::fatal(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
/// 🧾️ The outcome of a prepared plan: its compact diff, or the refusal naming why it cannot be built.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn plan_outcome(plan: Result<xml_address::PptxPlan, String>) -> protocol::MutationOutcome<PptxDiff> {
    match plan {
        Ok(plan) => protocol::MutationOutcome::new(plan.diff),
        Err(message) => protocol::MutationOutcome::refuse(protocol::OutcomeCode::TargetMismatch, message, Vec::<String>::new()),
    }
}

/// ↩️ The exact inverse of a prepared plan; a refused plan has nothing to undo.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn plan_inverse(plan: Result<xml_address::PptxPlan, String>) -> Vec<PptxMutation> {
    plan.map(|plan| vec![plan.inverse]).unwrap_or_default()
}

/// 🧮️ The node-level mutations that carry `base` to `next`: every XML part whose root changed becomes a `replace-xml-node` of that root. `None` when
/// `next` changes anything else (the schema, the OPC layer, which parts exist), detected by replaying the result against `base`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn net_mutations(base: &PptxSnapshot, next: &PptxSnapshot) -> Option<Vec<PptxMutation>> {
    let mut leaves = Vec::new();
    for part in &next.xml_parts {
        let before = base.xml_parts.iter().find(|candidate| candidate.path == part.path)?;
        if before.document.root != part.document.root {
            leaves.push(PptxMutation::ReplaceXmlNode(replace_xml_node::ReplaceXmlNode { address: xml_address::pptx_xml_address(base, &part.path, Vec::new()).ok()?, node: part.document.root.clone()? }));
        }
    }
    let mut state = base.clone();
    for leaf in &leaves {
        if !apply_pptx_mutation(&mut state, leaf).messages().is_empty() {
            return None;
        }
    }
    (state == *next).then_some(leaves)
}
/// 🌳 The XML diff that rewrites every attribute value equal to a member of `from` into `to` anywhere under `node` -- a namespace
/// declaration is an ordinary attribute, so one walk covers `xmlns`, `xmlns:a` and whatever prefixed alias a deck uses. `None` when
/// nothing under `node` declares one of `from`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn retarget_attribute_values_diff(node: &semio_s_artifact_stdio_xml::schema::snapshot::XmlNode, from: &[&str], to: &str) -> Option<semio_s_artifact_stdio_xml::schema::diff::XmlNodeDiff> {
    use semio_s_artifact_stdio_xml::schema::diff::{XmlAttrModified, XmlAttributesDiff, XmlChildModified, XmlChildrenDiff, XmlElementDiff, XmlNodeDiff};
    let semio_s_artifact_stdio_xml::schema::snapshot::XmlNode::Element { attrs, children, .. } = node else { return None };
    let modified: Vec<XmlAttrModified> = attrs.iter().filter(|attr| from.contains(&attr.value.as_str()) && attr.value != to).map(|attr| XmlAttrModified { name: attr.name.clone(), value: to.to_string() }).collect();
    let attributes = (!modified.is_empty()).then(|| XmlAttributesDiff { order: attrs.iter().map(|attr| attr.name.clone()).collect(), removed: Vec::new(), modified, added: Vec::new() });
    let nested: Vec<XmlChildModified> = children.iter().enumerate().filter_map(|(index, child)| retarget_attribute_values_diff(child, from, to).map(|diff| XmlChildModified { index, diff })).collect();
    let children = (!nested.is_empty()).then(|| XmlChildrenDiff { removed: Vec::new(), modified: nested, added: Vec::new() });
    (attributes.is_some() || children.is_some()).then(|| XmlNodeDiff::Element(XmlElementDiff { name: None, attributes, children }))
}

/// 🧾️ The diff that applies one XML `root` edit to each named part: `edits` pairs a part path with the root diff it takes.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn root_edits_diff(edits: Vec<(String, semio_s_artifact_stdio_xml::schema::diff::XmlNodeDiff)>) -> PptxDiff {
    if edits.is_empty() {
        return PptxDiff::default();
    }
    let modified = edits
        .into_iter()
        .map(|(key, root)| crate::schema::diff::NamedModified { key, diff: crate::schema::diff::PptxXmlPartDiff { content_type: None, document: Some(semio_s_artifact_stdio_xml::schema::diff::XmlDiff { root: Some(root), ..Default::default() }) } })
        .collect();
    PptxDiff { schema: None, opc: None, xml_parts: Some(crate::schema::diff::NamedTripleDiff { modified, ..Default::default() }) }
}

/// 🌿️ The attribute edit that sets (`Some`) or removes (`None`) attribute `name` on the ROOT element of `document`; `None` when nothing changes.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn root_attribute_diff(document: &semio_s_artifact_stdio_xml::schema::snapshot::XmlDocument, name: &str, value: Option<&str>) -> Option<semio_s_artifact_stdio_xml::schema::diff::XmlNodeDiff> {
    use semio_s_artifact_stdio_xml::schema::diff::{XmlAttrAdded, XmlAttrModified, XmlAttributesDiff, XmlElementDiff, XmlNodeDiff};
    let Some(semio_s_artifact_stdio_xml::schema::snapshot::XmlNode::Element { attrs, .. }) = document.root.as_ref() else { return None };
    let existing = attrs.iter().find(|attr| attr.name == name);
    let mut order: Vec<String> = attrs.iter().map(|attr| attr.name.clone()).collect();
    let attributes = match (existing, value) {
        (Some(attr), Some(value)) if attr.value != value => XmlAttributesDiff { order, modified: vec![XmlAttrModified { name: name.into(), value: value.into() }], ..Default::default() },
        (None, Some(value)) => {
            order.push(name.to_string());
            XmlAttributesDiff { order, added: vec![XmlAttrAdded { name: name.into(), value: value.into() }], ..Default::default() }
        }
        (Some(_), None) => {
            order.retain(|attr| attr != name);
            XmlAttributesDiff { order, removed: vec![name.into()], ..Default::default() }
        }
        _ => return None,
    };
    Some(XmlNodeDiff::Element(XmlElementDiff { name: None, attributes: Some(attributes), children: None }))
}

/// 🌿️ The children edit that appends `node` to (`Some`) or strips every child named `name` from (`None`) the ROOT element of `document`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn root_children_diff(document: &semio_s_artifact_stdio_xml::schema::snapshot::XmlDocument, append: Option<semio_s_artifact_stdio_xml::schema::snapshot::XmlNode>, strip: Option<&str>) -> Option<semio_s_artifact_stdio_xml::schema::diff::XmlNodeDiff> {
    use semio_s_artifact_stdio_xml::schema::diff::{XmlChildAdded, XmlChildrenDiff, XmlElementDiff, XmlNodeDiff};
    use semio_s_artifact_stdio_xml::schema::snapshot::XmlNode;
    let Some(XmlNode::Element { children, .. }) = document.root.as_ref() else { return None };
    let removed: Vec<usize> = strip.map(|name| children.iter().enumerate().filter(|(_, child)| matches!(child, XmlNode::Element { name: actual, .. } if actual == name)).map(|(index, _)| index).collect()).unwrap_or_default();
    let added: Vec<XmlChildAdded> = append.map(|item| vec![XmlChildAdded { index: children.len(), item }]).unwrap_or_default();
    (!removed.is_empty() || !added.is_empty()).then(|| XmlNodeDiff::Element(XmlElementDiff { name: None, attributes: None, children: Some(XmlChildrenDiff { removed, modified: Vec::new(), added }) }))
}

/// 🧩️ The diff that adds XML part `path` of `content_type` with `document` at `index` (append when `None`), together with its content-type
/// override. `None` when a part of that name already exists.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn insert_xml_part_diff(base: &PptxSnapshot, path: &str, content_type: &str, document: &semio_s_artifact_stdio_xml::schema::snapshot::XmlDocument, index: Option<usize>) -> Option<PptxDiff> {
    use crate::schema::diff::{NamedModified, NamedTripleDiff, PptxOpcContentTypesDiff, PptxOpcDiff};
    let key = path.trim_start_matches('/');
    if base.xml_parts.iter().any(|part| part.path == key) || base.opc.part(key).is_some() {
        return None;
    }
    let position = index.map_or(base.xml_parts.len(), |index| index.min(base.xml_parts.len()));
    let mut order: Vec<String> = base.xml_parts.iter().map(|part| part.path.clone()).collect();
    order.insert(position, key.to_string());
    let appended = position == base.xml_parts.len();
    let part = crate::schema::snapshot::PptxXmlPart { path: key.to_string(), content_type: content_type.to_string(), document: document.clone() };
    let name = format!("/{key}");
    let present = base.opc.content_types.overrides.iter().any(|(existing, _)| *existing == name);
    let overrides = if present {
        let current = base.opc.content_types.overrides.iter().find(|(existing, _)| *existing == name).map(|(_, value)| value.clone());
        (current.as_deref() != Some(content_type)).then(|| NamedTripleDiff { modified: vec![NamedModified { key: name.clone(), diff: content_type.to_string() }], ..Default::default() })
    } else {
        Some(NamedTripleDiff { added: vec![(name, content_type.to_string())], ..Default::default() })
    };
    Some(PptxDiff {
        schema: None,
        opc: overrides.map(|overrides| PptxOpcDiff { content_types: Some(PptxOpcContentTypesDiff { defaults: None, overrides: Some(overrides) }), ..Default::default() }),
        xml_parts: Some(NamedTripleDiff { added: vec![part], order: if appended { Vec::new() } else { order }, ..Default::default() }),
    })
}

/// 🧩️ The diff that removes XML part `path` and its content-type override; `None` when no such part exists.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn remove_xml_part_diff(base: &PptxSnapshot, path: &str) -> Option<PptxDiff> {
    use crate::schema::diff::{NamedTripleDiff, PptxOpcContentTypesDiff, PptxOpcDiff};
    let key = path.trim_start_matches('/');
    base.xml_parts.iter().find(|part| part.path == key)?;
    let name = format!("/{key}");
    let has_override = base.opc.content_types.overrides.iter().any(|(existing, _)| *existing == name);
    Some(PptxDiff {
        schema: None,
        opc: has_override.then(|| PptxOpcDiff { content_types: Some(PptxOpcContentTypesDiff { defaults: None, overrides: Some(NamedTripleDiff { removed: vec![name], ..Default::default() }) }), ..Default::default() }),
        xml_parts: Some(NamedTripleDiff { removed: vec![key.to_string()], ..Default::default() }),
    })
}
//#endregion 🔖️MutationTrait

//#region 🔖️DemoCases
/// 🧪️ FG-wave: representative `PptxMutation` values -- one per variant -- the single source of
/// truth reused by this file's own `mutation_diff_law`/`inverse_law`/`op_text_binary_roundtrip_law`
/// tests AND by `⚙️engine/🦀️.rs`'s `ops_grammar_conformance_law`/`protocol_walk_law`
/// conformance tests, same shape `📜️docx/…/🧬️mutations/🦀️.rs`'s own
/// `demo_mutation_cases()` establishes.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_fixture() -> PptxSnapshot {
    crate::standards::v_ecma_376::subsets::base::schema::construction::minimal::build_minimal_pptx(crate::schema::snapshot::PptxPresentation {
        slides: vec![
            PptxSlide { shapes: vec![PptxShape::TextBox { text_frame: vec![PptxParagraph::text("first")], position: PptxTransform { x: 0, y: 0, cx: 100, cy: 100 } }] },
            PptxSlide { shapes: vec![PptxShape::TextBox { text_frame: vec![PptxParagraph::text("second")], position: PptxTransform::default() }] },
        ],
    })
}

#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_mutation_cases() -> Vec<PptxMutation> {
    let fixture = demo_fixture();
    let slides = xml_address::pptx_slides(&fixture).expect("canonical demo slides");
    let first_shape = slides[0].shapes[0].address.clone();
    let second_slide = slides[1].address.clone();
    let slide_entry = xml_address::resolve_pptx_xml_address(&fixture, &slides[0].address.entry).expect("canonical slide entry").clone();
    let shape_node = xml_address::resolve_pptx_xml_address(&fixture, &first_shape.node).expect("canonical shape node").clone();
    let shape_node_for_replace = shape_node.clone();
    vec![
        PptxMutation::InsertSlide(insert_slide::InsertSlide { vacancy: xml_address::pptx_slide_vacancy(&fixture, 1).expect("slide vacancy"), entry: slide_entry }),
        PptxMutation::RemoveSlide(remove_slide::RemoveSlide { address: slides[0].address.clone() }),
        PptxMutation::MoveSlide(move_slide::MoveSlide { address: second_slide, destination_index: 0 }),
        PptxMutation::InsertShape(insert_shape::InsertShape { vacancy: xml_address::pptx_shape_vacancy(&fixture, &slides[0].address, 1).expect("shape vacancy"), shape: shape_node }),
        PptxMutation::RemoveShape(remove_shape::RemoveShape { address: first_shape.clone() }),
        PptxMutation::SetShapeText(set_shape_text::SetShapeText { address: first_shape.clone(), text: "z".into() }),
        PptxMutation::SetShapePosition(set_shape_position::SetShapePosition { address: first_shape.clone(), position: PptxTransform { x: 5, y: 6, cx: 7, cy: 8 } }),
        PptxMutation::ReplaceXmlNode(replace_xml_node::ReplaceXmlNode { address: first_shape.node, node: shape_node_for_replace }),
    ]
}
//#endregion 🔖️DemoCases

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
