//! 🧬️ SemioModelMutation — named-variant vocabulary (gif 89a / docx precedent): a sparse
//! `SetSnapshot` plus insert/remove/set per collection (spatial/elements/relations). Every
//! variant's `diff()`/`inverse()` is HAND-WRITTEN below (schema-design.md: apply-and-capture via
//! clone+apply+re-diff is banned -- each variant constructs its `SemioModelDiff` directly).

use crate::standards::v1::subsets::base::schema::geometry::{SemioQuaternion, SemioTransform};
use crate::standards::v1::subsets::base::schema::triples::{NamedModified, NamedTripleDiff};


use crate::standards::v1::subsets::model::schema::diff::{diff_set_snapshot, ModelRelationDiff, SemioModelDiff, SemioModelElementDiff, SpatialNodeDiff};

























use crate::standards::v1::subsets::model::schema::snapshot::{ElementClass, GeometryRef, ModelRelation, PropertySet, RelationKind, SemioModelElement, SemioModelSnapshot, SpatialKind, SpatialNode};
use protocol::Mutation;
/// 🔧️ Unconditional — the non-test `impl protocol::OpBinary for SemioModelMutation` block below
/// calls `Self::parse_op(...)` via trait method syntax, which needs `OpText` in scope in
/// production code too, not merely under `#[cfg(test)]` (same fix `stdio.semio.flow`'s own
/// mutations facet needed).

use semio_s_artifact_stdio_contract::deserialize_double_option;

//#region 🔖️Mutation
//#region 🔖️Leaves
#[path = "🧱insert-element/🦀️.rs"]
pub mod insert_element;
#[path = "🪢insert-relation/🦀️.rs"]
pub mod insert_relation;
#[path = "🏗️insert-spatial-node/🦀️.rs"]
pub mod insert_spatial_node;
#[path = "🔨remove-element/🦀️.rs"]
pub mod remove_element;
#[path = "✂️remove-relation/🦀️.rs"]
pub mod remove_relation;
#[path = "🕳️remove-spatial-node/🦀️.rs"]
pub mod remove_spatial_node;
#[path = "🎛️set-element/🦀️.rs"]
pub mod set_element;
#[path = "🔧️set-relation/🦀️.rs"]
pub mod set_relation;
#[path = "🧭set-spatial-node/🦀️.rs"]
pub mod set_spatial_node;
#[path = "✋️drag-elements/🦀️.rs"]
pub mod drag_elements;
#[path = "🔄️rotate-elements/🦀️.rs"]
pub mod rotate_elements;
#[path = "🔍️scale-elements/🦀️.rs"]
pub mod scale_elements;
//#endregion 🔖️Leaves

/// 📐️ Typed mutation for this subset. `NoMutation` was dropped: `#[derive(dsl::Mutations)]` requires
/// every variant to wrap exactly one leaf payload and a unit variant wraps none (same consequence
/// tiff's baseline migration reached — see
/// `🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧱️baseline/🧬️schema/🧬️mutations/🦀️.rs`).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[mutations(snapshot = SemioModelSnapshot, diff = SemioModelDiff, schema = "SemioModelMutation")]
#[value(tag = "mutation", rename_all = "camelCase")]
pub enum SemioModelMutation {
    InsertSpatialNode(insert_spatial_node::InsertSpatialNode),
    RemoveSpatialNode(remove_spatial_node::RemoveSpatialNode),
    SetSpatialNode(set_spatial_node::SetSpatialNode),
    InsertElement(insert_element::InsertElement),
    RemoveElement(remove_element::RemoveElement),
    SetElement(set_element::SetElement),
    InsertRelation(insert_relation::InsertRelation),
    RemoveRelation(remove_relation::RemoveRelation),
    SetRelation(set_relation::SetRelation),
    DragElements(drag_elements::DragElements),
    RotateElements(rotate_elements::RotateElements),
    ScaleElements(scale_elements::ScaleElements),
}

/// 🏷️ This subset's DECLARED mutation vocabulary, kebab-case, in enum declaration order — the one
/// list the repository test platform's completeness gate measures `🏛️mutate-semio-model` against
/// (catalog `semio-v1-model` in `../../🔮️oracles/🔣️.json`). 
/// `kinds_match_the_enum_and_the_catalog` keeps it honest against the enum, the manifest and the
/// `💾️binary/📡️.protocol.semio` records that carry each kind's wire tag.
pub const KINDS: &[&str] = &["insert-spatial-node", "remove-spatial-node", "set-spatial-node", "insert-element", "remove-element", "set-element", "insert-relation", "remove-relation", "set-relation", "drag-elements", "rotate-elements", "scale-elements"];

/// 🧮️ Pure diff face of [`Mutation::diff`], named only in this subset's own reachable types (`protocol` is a private
/// `extern crate` alias, so an owner-root test adapter cannot bring the `Mutation` trait into scope).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_semio_model_mutation(mutation: &SemioModelMutation, base: &SemioModelSnapshot) -> protocol::MutationOutcome<SemioModelDiff> {
    <SemioModelMutation as protocol::Mutation<SemioModelSnapshot>>::diff(mutation, base)
}


/// ↩️ `SemioModelMutation`'s own computed inverse, reachable from OUTSIDE this crate. `protocol` is
/// a private `extern crate semio_framework_os_kernel as protocol` alias in `🦀️.rs`, so an
/// external caller — an owner-root test adapter is exactly that — cannot bring `protocol::Mutation`
/// into scope and therefore cannot call the trait method at all. This wrapper's signature names
/// only types this subset already exports (`kit`'s precedent for the same structural gap).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn semio_model_mutation_inverse(mutation: &SemioModelMutation, base: &SemioModelSnapshot) -> Result<Vec<SemioModelMutation>, semio_framework_value::ValueError> {
    Ok({
    <SemioModelMutation as Mutation<SemioModelSnapshot>>::inverse(mutation, base)?

    })
}
//#endregion 🔖️Mutation




//#endregion 🔖️MutationTrait

//#region 🔖️RelativePlacement
/// 🧭️ The diff of one relative placement edit: `edit` rewrites the BASE placement of every addressed element, so the leaf
/// replays on any base. An empty or repeated target list is a Fatal `mutation.invariant`, no addressed element left is
/// `mutation.target-missing`, a target the model lacks is skipped as `mutation.partial`, an `identity` motion is
/// `mutation.no-op`.
pub(crate) fn relative_placement_diff(targets: &[String], identity: bool, base: &SemioModelSnapshot, edit: impl Fn(&mut SemioTransform)) -> protocol::MutationOutcome<SemioModelDiff> {
    if targets.is_empty() || targets.iter().enumerate().any(|(at, id)| targets[..at].contains(id)) {
        return protocol::MutationOutcome::fatal("mutation.invariant", "targets must name at least one element and never one twice", targets.to_vec());
    }
    let missing: Vec<String> = targets.iter().filter(|id| !base.elements.iter().any(|element| element.id == **id)).cloned().collect();
    if missing.len() == targets.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("none of the {} target(s) is an element of this model", targets.len()), targets.to_vec());
    }
    let partial: Vec<protocol::MutationMessage> =
        (!missing.is_empty()).then(|| protocol::MutationMessage::warning("mutation.partial", format!("{} of {} target(s) skipped (not in this model): {}", missing.len(), targets.len(), missing.join(", "))).at(missing)).into_iter().collect();
    if identity {
        return protocol::MutationOutcome::new(SemioModelDiff::default()).absorb_messages(partial.into_iter().chain([protocol::MutationMessage::warning("mutation.no-op", "an identity motion moves nothing").at(targets.to_vec())]));
    }
    let modified = base
        .elements
        .iter()
        .filter(|element| targets.contains(&element.id))
        .map(|element| {
            let mut placement = element.placement;
            edit(&mut placement);
            NamedModified { key: element.id.clone(), diff: SemioModelElementDiff { placement: Some(placement), ..Default::default() } }
        })
        .collect();
    protocol::MutationOutcome::new(SemioModelDiff { elements: Some(NamedTripleDiff { modified, ..Default::default() }), ..Default::default() }).absorb_messages(partial)
}

/// ↩️ The exact undo of a relative placement edit: one absolute `set-element` placement per addressed element carrying its
/// BASE placement — never a negated motion that would accumulate float error. Nothing for an identity or invalid motion.
pub(crate) fn relative_placement_inverse(targets: &[String], inert: bool, base: &SemioModelSnapshot) -> Vec<SemioModelMutation> {
    if inert {
        return Vec::new();
    }
    let mut seen = std::collections::HashSet::new();
    targets
        .iter()
        .filter(|id| seen.insert(id.as_str()))
        .filter_map(|id| base.elements.iter().find(|element| element.id == *id))
        .map(|element| SemioModelMutation::SetElement(set_element::SetElement { id: element.id.clone(), class: None, placement: Some(element.placement), geometry: None, spatial_id: None, psets: None }))
        .collect()
}

/// 🌀️ The Hamilton product `left ⊗ right` — a world-axis turn composed onto an element's own orientation.
pub fn quaternion_product(left: SemioQuaternion, right: SemioQuaternion) -> SemioQuaternion {
    let SemioQuaternion { x: lx, y: ly, z: lz, w: lw } = left;
    let SemioQuaternion { x: rx, y: ry, z: rz, w: rw } = right;
    SemioQuaternion { x: lw * rx + lx * rw + ly * rz - lz * ry, y: lw * ry - lx * rz + ly * rw + lz * rx, z: lw * rz + lx * ry - ly * rx + lz * rw, w: lw * rw - lx * rx - ly * ry - lz * rz }
}

/// 🔢️ A label number `(en, de)`: two decimals at most, trailing zeros dropped, the German decimal comma.
fn number_label(value: f64) -> (String, String) {
    let rounded = (value * 100.0).round() / 100.0;
    let text = format!("{:.2}", if rounded == 0.0 { 0.0 } else { rounded });
    let en = text.trim_end_matches('0').trim_end_matches('.').to_string();
    let de = en.replace('.', ",");
    (en, de)
}

/// 📐️ A label vector `(en, de)`: "(1, 0, 2.5)" and "(1; 0; 2,5)".
fn vector_label(values: [f64; 3]) -> (String, String) {
    let [x, y, z] = values.map(number_label);
    (format!("({}, {}, {})", x.0, y.0, z.0), format!("({}; {}; {})", x.1, y.1, z.1))
}

/// 🔠️ A label's counted noun `(en, de)`: "1 element" / "1 Element", "3 elements" / "3 Elemente".
fn element_count_label(count: usize) -> (String, String) {
    match count {
        1 => ("1 element".to_string(), "1 Element".to_string()),
        count => (format!("{count} elements"), format!("{count} Elemente")),
    }
}




//#endregion 🔖️RelativePlacement

//#region 🔖️OpCodecs














//#endregion 🔖️OpCodecs

//#region 🔖️Demo
/// 🌱 Shared fixture helpers + representative `SemioModelMutation` cases (one per variant) —
/// single source of truth for this facet's own tests AND `ops_grammar_conformance_law`/
/// `protocol_walk_law` in `🎹️composer/🦀️.rs`.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn sample_transform() -> SemioTransform {
    use crate::standards::v1::subsets::base::schema::geometry::{SemioPoint3, SemioQuaternion};
    SemioTransform { translation: SemioPoint3 { x: 5.0, y: 6.0, z: 7.0 }, rotation: SemioQuaternion::default(), scale: SemioPoint3 { x: 1.0, y: 1.0, z: 1.0 } }
}

#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn fixture() -> SemioModelSnapshot {
    let mut snap = SemioModelSnapshot::default();
    snap.spatial.push(SpatialNode { id: "s1".into(), kind: SpatialKind::Site, name: "Site".into(), parent_id: None, placement: SemioTransform::identity() });
    snap.elements.push(SemioModelElement { id: "e1".into(), class: ElementClass::Wall, placement: SemioTransform::identity(), geometry: GeometryRef::None, spatial_id: None, psets: vec![] });
    snap.relations.push(ModelRelation { id: "r1".into(), kind: RelationKind::Aggregates, from: "e1".into(), to: "s1".into() });
    snap
}

#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_mutation_cases() -> Vec<SemioModelMutation> {
    let base = fixture();
    vec![
        SemioModelMutation::InsertSpatialNode(insert_spatial_node::InsertSpatialNode { node: SpatialNode { id: "s2".into(), kind: SpatialKind::Space, name: "Room".into(), parent_id: None, placement: SemioTransform::identity() } }),
        SemioModelMutation::RemoveSpatialNode(remove_spatial_node::RemoveSpatialNode { id: "s1".into() }),
        SemioModelMutation::SetSpatialNode(set_spatial_node::SetSpatialNode { id: "s1".into(), kind: Some(SpatialKind::Storey), name: None, parent_id: Some(Some("root".into())), placement: None }),
        SemioModelMutation::InsertElement(insert_element::InsertElement {
            element: SemioModelElement { id: "e2".into(), class: ElementClass::Beam, placement: SemioTransform::identity(), geometry: GeometryRef::None, spatial_id: None, psets: vec![] },
        }),
        SemioModelMutation::RemoveElement(remove_element::RemoveElement { id: "e1".into() }),
        SemioModelMutation::SetElement(set_element::SetElement { id: "e1".into(), class: None, placement: None, geometry: Some(GeometryRef::None), spatial_id: Some(None), psets: None }),
        SemioModelMutation::InsertRelation(insert_relation::InsertRelation { relation: ModelRelation { id: "r2".into(), kind: RelationKind::Other { label: "custom".into() }, from: "e1".into(), to: "s1".into() } }),
        SemioModelMutation::RemoveRelation(remove_relation::RemoveRelation { id: "r1".into() }),
        SemioModelMutation::SetRelation(set_relation::SetRelation { id: "r1".into(), kind: Some(RelationKind::ConnectsTo), from: None, to: None }),
        SemioModelMutation::DragElements(drag_elements::DragElements { targets: vec!["e1".into()], offset: [1.5, 2.0, 0.25] }),
        SemioModelMutation::RotateElements(rotate_elements::RotateElements { targets: vec!["e1".into()], axis: [0.0, 0.0, 1.0], angle: std::f64::consts::FRAC_PI_2 }),
        SemioModelMutation::ScaleElements(scale_elements::ScaleElements { targets: vec!["e1".into()], factors: [2.0, 2.0, 0.5] }),
    ]
}
//#endregion 🔖️Demo

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests

#[cfg(test)]
use protocol::{OpBinary,OpText};
