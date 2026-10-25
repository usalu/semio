//! 🧬️ SemioModelMutation — named-variant vocabulary (gif 89a / docx precedent): a sparse
//! `SetSnapshot` plus insert/remove/set per collection (spatial/elements/relations). Every
//! variant's `diff()`/`inverse()` is HAND-WRITTEN below (schema-design.md: apply-and-capture via
//! clone+apply+re-diff is banned -- each variant constructs its `SemioModelDiff` directly).

use semio_s_artifact_stdio_semio_base::schema::geometry::SemioPoint3;
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioQuaternion;
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioTransform;
use semio_s_artifact_stdio_semio_base::schema::triples::NamedModified;
use semio_s_artifact_stdio_semio_base::schema::triples::NamedTripleDiff;


use crate::schema::diff::{ModelRelationDiff, SemioModelDiff, SemioModelElementDiff, SpatialNodeDiff};

























use crate::schema::snapshot::{ElementClass, GeometryRef, ModelRelation, PropertySet, RelationKind, SemioModelElement, SemioModelSnapshot, SpatialKind, SpatialNode};
use protocol::{ColdMutation, Mutation};
/// 🔧️ Unconditional — the non-test `impl protocol::OpBinary for SemioModelMutation` block below
/// calls `Self::parse_op(...)` via trait method syntax, which needs `OpText` in scope in
/// production code too, not merely under `#[cfg(test)]` (same fix `stdio.semio.flow`'s own
/// mutations facet needed).

use semio_s_artifact_stdio_contract::deserialize_double_option;

//#region 🔖️Mutation
//#region 🔖️Leaves
pub use semio_s_artifact_stdio_semio_model_examples::schema::mutations::support;
pub use support::{element_count_label, number_label, quaternion_product, relative_placement_diff, vector_label};
pub use semio_s_artifact_stdio_semio_model_mut_family::schema::mutations::insert_element;
pub use semio_s_artifact_stdio_semio_model_mut_family::schema::mutations::insert_relation;
pub use semio_s_artifact_stdio_semio_model_mut_family::schema::mutations::insert_spatial_node;
pub use semio_s_artifact_stdio_semio_model_mut_family::schema::mutations::remove_element;
pub use semio_s_artifact_stdio_semio_model_mut_family::schema::mutations::remove_relation;
pub use semio_s_artifact_stdio_semio_model_mut_family::schema::mutations::remove_spatial_node;
pub use semio_s_artifact_stdio_semio_model_examples::schema::mutations::set_element;
pub use semio_s_artifact_stdio_semio_model_mut_family::schema::mutations::set_relation;
pub use semio_s_artifact_stdio_semio_model_mut_family::schema::mutations::set_spatial_node;
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
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutations(snapshot = SemioModelSnapshot, diff = SemioModelDiff, schema = "SemioModelMutation", reference = SemioModelSnapshot, reference_diff = SemioModelDiff)]
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
    <SemioModelMutation as ColdMutation<SemioModelSnapshot>>::diff(mutation, base)
}


/// ↩️ `SemioModelMutation`'s own computed inverse, reachable from OUTSIDE this crate. `protocol` is
/// a private `extern crate semio_framework_os_kernel as protocol` alias in `🦀️.rs`, so an
/// external caller — an owner-root test adapter is exactly that — cannot bring `protocol::Mutation`
/// into scope and therefore cannot call the trait method at all. This wrapper's signature names
/// only types this subset already exports (`kit`'s precedent for the same structural gap).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn semio_model_mutation_inverse(mutation: &SemioModelMutation, base: &SemioModelSnapshot) -> Result<Vec<SemioModelMutation>, semio_framework_value::ValueError> {
    Ok({
    <SemioModelMutation as ColdMutation<SemioModelSnapshot>>::inverse(mutation, base)?

    })
}
//#endregion 🔖️Mutation




//#endregion 🔖️MutationTrait

//#region 🔖️RelativePlacement

/// ↩️ The exact undo of a relative placement edit: one absolute `set-element` placement per addressed element carrying its
/// BASE placement — never a negated motion that would accumulate float error. Nothing for an identity or invalid motion.
pub fn relative_placement_inverse<Op: From<set_element::SetElement>>(targets: &[String], inert: bool, base: &SemioModelSnapshot) -> Vec<Op> {
    if inert {
        return Vec::new();
    }
    let mut seen = std::collections::HashSet::new();
    targets
        .iter()
        .filter(|id| seen.insert(id.as_str()))
        .filter_map(|id| base.elements.iter().find(|element| element.id == *id))
        .map(|element| Op::from(set_element::SetElement { id: element.id.clone(), class: None, placement: Some(element.placement), geometry: None, spatial_id: None, psets: None }))
        .collect()
}








//#endregion 🔖️RelativePlacement


//#region 🔖️Demo
/// 🌱 Shared fixture helpers + representative `SemioModelMutation` cases (one per variant) —
/// single source of truth for this facet's own tests AND `ops_grammar_conformance_law`/
/// `protocol_walk_law` in `🎹️composer/🦀️.rs`.
#[cfg(any(test, feature = "testkit"))]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn sample_transform() -> SemioTransform {
    use semio_s_artifact_stdio_semio_base::schema::geometry::SemioPoint3;
    use semio_s_artifact_stdio_semio_base::schema::geometry::SemioQuaternion;
    SemioTransform { translation: SemioPoint3 { x: 5.0, y: 6.0, z: 7.0 }, rotation: SemioQuaternion::default(), scale: SemioPoint3 { x: 1.0, y: 1.0, z: 1.0 } }
}

#[cfg(any(test, feature = "testkit"))]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn fixture() -> SemioModelSnapshot {
    let mut snap = SemioModelSnapshot::default();
    snap.spatial.push(SpatialNode { id: "s1".into(), kind: SpatialKind::Site, name: "Site".into(), parent_id: None, placement: SemioTransform::identity() });
    snap.elements.push(SemioModelElement { id: "e1".into(), class: ElementClass::Wall, placement: SemioTransform::identity(), geometry: GeometryRef::None, spatial_id: None, psets: vec![] });
    snap.relations.push(ModelRelation { id: "r1".into(), kind: RelationKind::Aggregates, from: "e1".into(), to: "s1".into() });
    snap
}

#[cfg(any(test, feature = "testkit"))]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn demo_mutation_cases() -> Vec<SemioModelMutation> {
    let _base = fixture();
    vec![
        SemioModelMutation::InsertSpatialNode(insert_spatial_node::InsertSpatialNode { node: SpatialNode { id: "s2".into(), kind: SpatialKind::Space, name: "Room".into(), parent_id: None, placement: SemioTransform::identity() }, at: None }),
        SemioModelMutation::RemoveSpatialNode(remove_spatial_node::RemoveSpatialNode { id: "s1".into() }),
        SemioModelMutation::SetSpatialNode(set_spatial_node::SetSpatialNode { id: "s1".into(), kind: Some(SpatialKind::Storey), name: None, parent_id: Some(Some("root".into())), placement: None }),
        SemioModelMutation::InsertElement(insert_element::InsertElement {
            element: SemioModelElement { id: "e2".into(), class: ElementClass::Beam, placement: SemioTransform::identity(), geometry: GeometryRef::None, spatial_id: None, psets: vec![] }, at: None,
        }),
        SemioModelMutation::RemoveElement(remove_element::RemoveElement { id: "e1".into() }),
        SemioModelMutation::SetElement(set_element::SetElement { id: "e1".into(), class: None, placement: None, geometry: Some(GeometryRef::None), spatial_id: Some(None), psets: None }),
        SemioModelMutation::InsertRelation(insert_relation::InsertRelation { relation: ModelRelation { id: "r2".into(), kind: RelationKind::Other { label: "custom".into() }, from: "e1".into(), to: "s1".into() }, at: None }),
        SemioModelMutation::RemoveRelation(remove_relation::RemoveRelation { id: "r1".into() }),
        SemioModelMutation::SetRelation(set_relation::SetRelation { id: "r1".into(), kind: Some(RelationKind::ConnectsTo), from: None, to: None }),
        SemioModelMutation::DragElements(drag_elements::DragElements { targets: vec!["e1".into()], offset: [1.5, 2.0, 0.25] }),
        SemioModelMutation::RotateElements(rotate_elements::RotateElements { targets: vec!["e1".into()], axis: [0.0, 0.0, 1.0], angle: std::f64::consts::FRAC_PI_2 }),
        SemioModelMutation::ScaleElements(scale_elements::ScaleElements { targets: vec!["e1".into()], factors: [2.0, 2.0, 0.5] }),
    ]
}
//#endregion 🔖️Demo

//#region 🔖️Tests

//#endregion 🔖️Tests

#[cfg(test)]
use protocol::{OpBinary,OpText};
