//! 🧬️ PptxMutation — document mutation dispatch. Every variant's `diff()` is handcrafted (never
//! apply-and-capture) and every variant's `inverse()` is handcrafted, index-aware.

use crate::schema::diff::PptxDiff;
use crate::schema::snapshot::{PptxParagraph, PptxShape, PptxSlide, PptxTransform};
use crate::PptxSnapshot;
use protocol::Mutation;
use semio_s_artifact_stdio_xml::schema::snapshot::XmlNode;
use semio_s_artifact_stdio_zip::opc::diff::{OpcContentTypeEntriesDelta, OpcContentTypePatch, OpcContentTypeRow, OpcContentTypesDiff, OpcDiff};

//#region 🔖️Mutations
#[path = "🧭️edit-rules/🦀️.rs"]
pub mod edit_rules;
#[path = "🔗opc-layer/🦀️.rs"]
pub(crate) mod opc_layer;
#[path = "📎set-relationship/🦀️.rs"]
pub mod set_relationship;
#[path = "🧷remove-relationship/🦀️.rs"]
pub mod remove_relationship;
#[path = "📇set-content-type/🦀️.rs"]
pub mod set_content_type;
#[path = "🧺remove-content-type/🦀️.rs"]
pub mod remove_content_type;
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
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
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
    /// 📎 Writes one relationship of an owner part (inserted at `index`, or changed in place).
    SetRelationship(set_relationship::SetRelationship),
    /// 🧷 Removes one relationship of an owner part.
    RemoveRelationship(remove_relationship::RemoveRelationship),
    /// 📇 Writes one `[Content_Types].xml` entry (inserted at `index`, or changed in place).
    SetContentType(set_content_type::SetContentType),
    /// 🧺 Removes one `[Content_Types].xml` entry.
    RemoveContentType(remove_content_type::RemoveContentType),
}
//#endregion 🔖️Mutations

//#region 🔖️Kinds
/// 🏷️ The kebab-case spelling of every `PptxMutation` variant, in the same order the enum
/// declares them — this repository's mutation oracle registrations never parse this enum;
/// `kinds_matches_enum_variants_and_manifest` below is what keeps the two declarations honest
/// against each other.
pub const KINDS: &[&str] = &["insert-slide", "remove-slide", "move-slide", "insert-shape", "remove-shape", "set-shape-text", "set-shape-position", "replace-xml-node", "set-relationship", "remove-relationship", "set-content-type", "remove-content-type"];

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
        PptxMutation::SetRelationship(_) => "set-relationship",
        PptxMutation::RemoveRelationship(_) => "remove-relationship",
        PptxMutation::SetContentType(_) => "set-content-type",
        PptxMutation::RemoveContentType(_) => "remove-content-type",
    }
}
//#endregion 🔖️Kinds

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`: `let d = mutation.diff(&*snapshot); *snapshot =
/// d.apply(snapshot); d` -- the diff is the single semantics source, never a separate imperative
/// apply path (apply-and-capture is banned).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
#[cfg(test)]
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
        .map(|(id, root)| crate::schema::diff::PptxXmlPartModification { id, patch: crate::schema::diff::PptxXmlPartDiff { content_type: None, document: Some(semio_s_artifact_stdio_xml::schema::diff::XmlDiff { root: Some(root), ..Default::default() }) } })
        .collect();
    PptxDiff { schema: None, opc: None, xml_parts: Some(crate::schema::diff::PptxXmlPartsDelta { modified, ..Default::default() }) }
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

/// 🌿️ The children edit that inserts `insert.1` at final index `insert.0` of, or removes the children at the base indexes `remove` from, the ROOT element of `document`;
/// `None` when nothing changes. Insertion and removal are never combined in one call.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn root_children_diff(document: &semio_s_artifact_stdio_xml::schema::snapshot::XmlDocument, insert: Option<(usize, semio_s_artifact_stdio_xml::schema::snapshot::XmlNode)>, remove: &[usize]) -> Option<semio_s_artifact_stdio_xml::schema::diff::XmlNodeDiff> {
    use semio_s_artifact_stdio_xml::schema::diff::{XmlChildAdded, XmlChildrenDiff, XmlElementDiff, XmlNodeDiff};
    let Some(semio_s_artifact_stdio_xml::schema::snapshot::XmlNode::Element { .. }) = document.root.as_ref() else { return None };
    let added: Vec<XmlChildAdded> = insert.map(|(index, item)| vec![XmlChildAdded { index, item }]).unwrap_or_default();
    (!remove.is_empty() || !added.is_empty()).then(|| XmlNodeDiff::Element(XmlElementDiff { name: None, attributes: None, children: Some(XmlChildrenDiff { removed: remove.to_vec(), modified: Vec::new(), added }) }))
}

/// 🧩️ The diff that adds XML part `path` of `content_type` with `document` at `index` (append when `None`), together with its content-type
/// override. `None` when a part of that name already exists.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn insert_xml_part_diff(base: &PptxSnapshot, path: &str, content_type: &str, document: &semio_s_artifact_stdio_xml::schema::snapshot::XmlDocument, index: Option<usize>, override_index: Option<usize>) -> Option<PptxDiff> {
    use crate::schema::diff::PptxXmlPartsDelta;
    use semio_s_artifact_stdio_contract::list_delta::insertion_index;
    let key = path.trim_start_matches('/');
    if base.xml_parts.iter().any(|part| part.path == key) || base.opc.part(key).is_some() {
        return None;
    }
    let part = crate::schema::snapshot::PptxXmlPart { path: key.to_string(), content_type: content_type.to_string(), document: document.clone() };
    let name = format!("/{key}");
    let list = &base.opc.content_types.overrides;
    let overrides = match list.iter().position(|(existing, _)| *existing == name) {
        Some(at) => (list[at].1 != content_type).then(|| OpcContentTypeEntriesDelta::modification(&name, OpcContentTypePatch { content_type: Some(content_type.to_string()) })),
        None => Some(OpcContentTypeEntriesDelta::insertion(insertion_index(list.len(), override_index), OpcContentTypeRow { name, content_type: content_type.to_string() })),
    };
    Some(PptxDiff {
        schema: None,
        opc: overrides.map(|overrides| OpcDiff { content_types: Some(OpcContentTypesDiff { defaults: None, overrides: Some(overrides) }), ..Default::default() }),
        xml_parts: Some(PptxXmlPartsDelta::insertion(insertion_index(base.xml_parts.len(), index), part)),
    })
}

/// 🧩️ The diff that removes XML part `path` and its content-type override; `None` when no such part exists.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn remove_xml_part_diff(base: &PptxSnapshot, path: &str) -> Option<PptxDiff> {
    use crate::schema::diff::PptxXmlPartsDelta;
    let key = path.trim_start_matches('/');
    let at = base.xml_parts.iter().position(|part| part.path == key)?;
    let name = format!("/{key}");
    let overrides = &base.opc.content_types.overrides;
    let content_types = overrides.iter().position(|(existing, _)| *existing == name).map(|position| OpcContentTypesDiff { defaults: None, overrides: Some(OpcContentTypeEntriesDelta::removal_by_id(name.clone(), position)) });
    Some(PptxDiff { schema: None, opc: content_types.map(|content_types| OpcDiff { content_types: Some(content_types), ..Default::default() }), xml_parts: Some(PptxXmlPartsDelta::removal_by_id(key, at)) })
}
/// 🧭️ Where XML part `path` and its explicit content-type override sit in their lists: `(part index, override index)`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn xml_part_positions(base: &PptxSnapshot, path: &str) -> Option<(usize, Option<usize>)> {
    let key = path.trim_start_matches('/');
    let part = base.xml_parts.iter().position(|part| part.path == key)?;
    let name = format!("/{key}");
    Some((part, base.opc.content_types.overrides.iter().position(|(existing, _)| *existing == name)))
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
    opc_layer::with_demo_entries(crate::standards::v_ecma_376::subsets::base::schema::construction::minimal::build_minimal_pptx(crate::schema::snapshot::PptxPresentation {
        slides: vec![
            PptxSlide { shapes: vec![PptxShape::TextBox { text_frame: vec![PptxParagraph::text("first")], position: PptxTransform { x: 0, y: 0, cx: 100, cy: 100 } }] },
            PptxSlide { shapes: vec![PptxShape::TextBox { text_frame: vec![PptxParagraph::text("second")], position: PptxTransform::default() }] },
        ],
    }))
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
    .into_iter()
    .chain(opc_layer::demo_cases())
    .collect()
}
//#endregion 🔖️DemoCases

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
