//! 🧬️ SemioModelMutation — named-variant vocabulary (gif 89a / docx precedent): a sparse
//! `SetSnapshot` plus insert/remove/set per collection (spatial/elements/relations). Every
//! variant's `diff()`/`inverse()` is HAND-WRITTEN below (schema-design.md: apply-and-capture via
//! clone+apply+re-diff is banned -- each variant constructs its `SemioModelDiff` directly).

use crate::artifacts::semio::standards::v1::engine::geometry::SemioTransform;
use crate::artifacts::semio::standards::v1::engine::triples::{NamedModified, NamedTripleDiff};
use crate::artifacts::semio::standards::v1::subsets::model::schema::diff::{diff_set_snapshot, ModelRelationDiff, SemioModelDiff, SemioModelElementDiff, SpatialNodeDiff};
use crate::artifacts::semio::standards::v1::subsets::model::schema::snapshot::{
    ElementClass, GeometryRef, ModelRelation, PropertySet, RelationKind, SemioModelElement, SemioModelSnapshot, SpatialKind, SpatialNode,
};
use protocol::Mutation;
#[cfg(test)]
use protocol::{OpBinary, OpText};
use serde::{Deserialize, Serialize};

//#region 🔖️DoubleOption
/// 🕳️ Standard double-`Option` serde workaround: with plain `#[serde(default)]`, a field typed
/// `Option<Option<T>>` can't distinguish "untouched" (key absent) from "cleared" (`Some(None)`,
/// key present with JSON `null`) — both collapse to the outer `None` on decode, because serde's
/// blanket `Deserialize for Option<T>` treats `null` as absence at ANY nesting depth. Combined with
/// `skip_serializing_if = "Option::is_none"` (so "untouched" omits the key entirely), this makes
/// key-PRESENT-with-`null` unambiguously mean `Some(None)`.
fn deserialize_double_option<'de, D, T>(deserializer: D) -> Result<Option<Option<T>>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: Deserialize<'de>,
{
    Option::<T>::deserialize(deserializer).map(Some)
}
//#endregion 🔖️DoubleOption

//#region 🔖️Mutation
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum SemioModelMutation {
    #[default]
    NoMutation,
    /// 🧩 Sparse full-state replace -- `diff()` is `SemioModelDiff::between`, never a
    /// `snapshot: Option<Snapshot>` full-replace slot (schema-design.md).
    SetSnapshot { snapshot: SemioModelSnapshot },
    InsertSpatialNode { node: SpatialNode },
    RemoveSpatialNode { id: String },
    SetSpatialNode {
        id: String,
        #[serde(default)] kind: Option<SpatialKind>,
        #[serde(default)] name: Option<String>,
        /// 🕳️ Tri-state (`None` = untouched, `Some(None)` = cleared, `Some(Some(_))` = set) — the
        /// classic double-`Option` serde footgun: without `skip_serializing_if`/`deserialize_with`,
        /// `Some(None)` and the untouched `None` both serialize to JSON `null` and collapse back to
        /// the SAME outer `None` on decode (`w2a-verify-report.md`'s model finding, same shape as
        /// `spatial_id` below).
        #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
        parent_id: Option<Option<String>>,
        #[serde(default)] placement: Option<SemioTransform>,
    },
    InsertElement { element: SemioModelElement },
    RemoveElement { id: String },
    SetElement {
        id: String,
        #[serde(default)] class: Option<ElementClass>,
        #[serde(default)] placement: Option<SemioTransform>,
        #[serde(default)] geometry: Option<GeometryRef>,
        /// 🕳️ Tri-state, same double-`Option` footgun/fix as `SetSpatialNode.parent_id` above —
        /// confirmed live by the verifier: `Some(None)` round-tripped to the outer `None` through
        /// `print_op`/`parse_op` before this fix.
        #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
        spatial_id: Option<Option<String>>,
        #[serde(default)] psets: Option<Vec<PropertySet>>,
    },
    InsertRelation { relation: ModelRelation },
    RemoveRelation { id: String },
    SetRelation {
        id: String,
        #[serde(default)] kind: Option<RelationKind>,
        #[serde(default)] from: Option<String>,
        #[serde(default)] to: Option<String>,
    },
}

impl Mutation<SemioModelSnapshot> for SemioModelMutation {
    type Diff = SemioModelDiff;

    fn diff(&self, base: &SemioModelSnapshot) -> Self::Diff {
        match self {
            SemioModelMutation::NoMutation => SemioModelDiff::default(),
            SemioModelMutation::SetSnapshot { snapshot } => diff_set_snapshot(base, snapshot),
            SemioModelMutation::InsertSpatialNode { node } => SemioModelDiff {
                spatial: Some(NamedTripleDiff { added: vec![node.clone()], ..Default::default() }),
                ..Default::default()
            },
            SemioModelMutation::RemoveSpatialNode { id } => SemioModelDiff {
                spatial: Some(NamedTripleDiff { removed: vec![id.clone()], ..Default::default() }),
                ..Default::default()
            },
            SemioModelMutation::SetSpatialNode { id, kind, name, parent_id, placement } => SemioModelDiff {
                spatial: Some(NamedTripleDiff {
                    modified: vec![NamedModified { key: id.clone(), diff: SpatialNodeDiff { kind: *kind, name: name.clone(), parent_id: parent_id.clone(), placement: *placement } }],
                    ..Default::default()
                }),
                ..Default::default()
            },
            SemioModelMutation::InsertElement { element } => SemioModelDiff {
                elements: Some(NamedTripleDiff { added: vec![element.clone()], ..Default::default() }),
                ..Default::default()
            },
            SemioModelMutation::RemoveElement { id } => SemioModelDiff {
                elements: Some(NamedTripleDiff { removed: vec![id.clone()], ..Default::default() }),
                ..Default::default()
            },
            SemioModelMutation::SetElement { id, class, placement, geometry, spatial_id, psets } => SemioModelDiff {
                elements: Some(NamedTripleDiff {
                    modified: vec![NamedModified { key: id.clone(), diff: SemioModelElementDiff { class: class.clone(), placement: *placement, geometry: geometry.clone(), spatial_id: spatial_id.clone(), psets: psets.clone() } }],
                    ..Default::default()
                }),
                ..Default::default()
            },
            SemioModelMutation::InsertRelation { relation } => SemioModelDiff {
                relations: Some(NamedTripleDiff { added: vec![relation.clone()], ..Default::default() }),
                ..Default::default()
            },
            SemioModelMutation::RemoveRelation { id } => SemioModelDiff {
                relations: Some(NamedTripleDiff { removed: vec![id.clone()], ..Default::default() }),
                ..Default::default()
            },
            SemioModelMutation::SetRelation { id, kind, from, to } => SemioModelDiff {
                relations: Some(NamedTripleDiff {
                    modified: vec![NamedModified { key: id.clone(), diff: ModelRelationDiff { kind: kind.clone(), from: from.clone(), to: to.clone() } }],
                    ..Default::default()
                }),
                ..Default::default()
            },
        }
    }

    fn inverse(&self, base: &SemioModelSnapshot) -> Vec<Self> {
        match self {
            SemioModelMutation::NoMutation => vec![SemioModelMutation::NoMutation],
            SemioModelMutation::SetSnapshot { .. } => vec![SemioModelMutation::SetSnapshot { snapshot: base.clone() }],

            SemioModelMutation::InsertSpatialNode { node } => vec![SemioModelMutation::RemoveSpatialNode { id: node.id.clone() }],
            SemioModelMutation::RemoveSpatialNode { id } => match base.spatial.iter().find(|n| &n.id == id) {
                Some(original) => vec![SemioModelMutation::InsertSpatialNode { node: original.clone() }],
                None => vec![SemioModelMutation::NoMutation],
            },
            SemioModelMutation::SetSpatialNode { id, kind, name, parent_id, placement } => match base.spatial.iter().find(|n| &n.id == id) {
                Some(original) => vec![SemioModelMutation::SetSpatialNode {
                    id: id.clone(),
                    kind: kind.as_ref().map(|_| original.kind),
                    name: name.as_ref().map(|_| original.name.clone()),
                    parent_id: parent_id.as_ref().map(|_| original.parent_id.clone()),
                    placement: placement.as_ref().map(|_| original.placement),
                }],
                None => vec![SemioModelMutation::NoMutation],
            },

            SemioModelMutation::InsertElement { element } => vec![SemioModelMutation::RemoveElement { id: element.id.clone() }],
            SemioModelMutation::RemoveElement { id } => match base.elements.iter().find(|e| &e.id == id) {
                Some(original) => vec![SemioModelMutation::InsertElement { element: original.clone() }],
                None => vec![SemioModelMutation::NoMutation],
            },
            SemioModelMutation::SetElement { id, class, placement, geometry, spatial_id, psets } => match base.elements.iter().find(|e| &e.id == id) {
                Some(original) => vec![SemioModelMutation::SetElement {
                    id: id.clone(),
                    class: class.as_ref().map(|_| original.class.clone()),
                    placement: placement.as_ref().map(|_| original.placement),
                    geometry: geometry.as_ref().map(|_| original.geometry.clone()),
                    spatial_id: spatial_id.as_ref().map(|_| original.spatial_id.clone()),
                    psets: psets.as_ref().map(|_| original.psets.clone()),
                }],
                None => vec![SemioModelMutation::NoMutation],
            },

            SemioModelMutation::InsertRelation { relation } => vec![SemioModelMutation::RemoveRelation { id: relation.id.clone() }],
            SemioModelMutation::RemoveRelation { id } => match base.relations.iter().find(|r| &r.id == id) {
                Some(original) => vec![SemioModelMutation::InsertRelation { relation: original.clone() }],
                None => vec![SemioModelMutation::NoMutation],
            },
            SemioModelMutation::SetRelation { id, kind, from, to } => match base.relations.iter().find(|r| &r.id == id) {
                Some(original) => vec![SemioModelMutation::SetRelation {
                    id: id.clone(),
                    kind: kind.as_ref().map(|_| original.kind.clone()),
                    from: from.as_ref().map(|_| original.from.clone()),
                    to: to.as_ref().map(|_| original.to.clone()),
                }],
                None => vec![SemioModelMutation::NoMutation],
            },
        }
    }
}

/// ▶️ Applies a mutation to `snapshot` in place, returning the diff (mirrors gif's
/// `apply_gif_mutation` convention — used by the builder's `mutate()` and the set-snapshot leaf).
pub fn apply_semio_model_mutation(snapshot: &mut SemioModelSnapshot, mutation: &SemioModelMutation) -> SemioModelDiff {
    let diff = <SemioModelMutation as Mutation<SemioModelSnapshot>>::diff(mutation, snapshot);
    *snapshot = <SemioModelDiff as protocol::MutationDiff<SemioModelSnapshot>>::apply(&diff, snapshot);
    diff
}
//#endregion 🔖️Mutation

//#region 🔖️OpCodecs
/// 🎙️ Handcrafted `OpText`/`OpBinary` — plain compact `serde_json` round-trip of the whole
/// tagged enum (one line of JSON per op), the same "JSON-pack passthrough" honesty boundary the
/// subset's own `ArtifactPack` impl uses. Deliberately NOT `#[derive(dsl::DslOps)]` +
/// `#[dsl(block)]` — that path requires every nested type in the mutation's field tree to itself
/// implement `dsl::DslField` (via `dsl::DslRecord`), a repo-wide framework capability this
/// hand-rolled vocabulary does not depend on (f6-final-summary.md §4: generics/tuple/nested-array
/// derive gaps).
impl protocol::OpText for SemioModelMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        serde_json::from_str(line).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))
    }
    fn print_op(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }
}

impl protocol::OpBinary for SemioModelMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        serde_json::to_vec(self).map_err(|e| protocol::ProtocolError::Io(e.to_string()))
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        serde_json::from_slice(bytes).map_err(|e| protocol::ProtocolError::Io(e.to_string()))
    }
}
//#endregion 🔖️OpCodecs

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::engine::geometry::{SemioPoint3, SemioQuaternion};

    fn sample_transform() -> SemioTransform {
        SemioTransform { translation: SemioPoint3 { x: 5.0, y: 6.0, z: 7.0 }, rotation: SemioQuaternion::default(), scale: SemioPoint3 { x: 1.0, y: 1.0, z: 1.0 } }
    }

    fn base_snapshot() -> SemioModelSnapshot {
        let mut snap = SemioModelSnapshot::default();
        snap.spatial.push(SpatialNode { id: "s1".into(), kind: SpatialKind::Site, name: "Site".into(), parent_id: None, placement: SemioTransform::identity() });
        snap.elements.push(SemioModelElement { id: "e1".into(), class: ElementClass::Wall, placement: SemioTransform::identity(), geometry: GeometryRef::None, spatial_id: None, psets: vec![] });
        snap.relations.push(ModelRelation { id: "r1".into(), kind: RelationKind::Aggregates, from: "e1".into(), to: "s1".into() });
        snap
    }

    /// 🧪️ mutation_diff_law + inverse_law, exercised for every non-trivial variant: `mutation.diff(base)`
    /// must equal what `apply_semio_model_mutation` actually applied, and applying the mutation's
    /// own `inverse()` must restore `base` exactly.
    fn assert_round_trips(base: &SemioModelSnapshot, mutation: SemioModelMutation) {
        let diff = <SemioModelMutation as Mutation<SemioModelSnapshot>>::diff(&mutation, base);
        let mut applied = base.clone();
        let produced = apply_semio_model_mutation(&mut applied, &mutation);
        assert_eq!(produced, diff, "diff() must match what apply_semio_model_mutation actually applied for {mutation:?}");
        let expected = <SemioModelDiff as protocol::MutationDiff<SemioModelSnapshot>>::apply(&diff, base);
        assert_eq!(applied, expected, "applying the mutation must equal applying its own diff for {mutation:?}");

        let inv = <SemioModelMutation as Mutation<SemioModelSnapshot>>::inverse(&mutation, base);
        let mut restored = applied.clone();
        for m in &inv {
            let _ = apply_semio_model_mutation(&mut restored, m);
        }
        assert_eq!(&restored, base, "inverse must restore the original base for {mutation:?}");
    }

    #[test]
    fn mutation_diff_law_and_inverse_law_cover_every_collection() {
        let base = base_snapshot();

        assert_round_trips(&base, SemioModelMutation::NoMutation);

        let mut swapped = base.clone();
        swapped.elements[0].class = ElementClass::Slab;
        assert_round_trips(&base, SemioModelMutation::SetSnapshot { snapshot: swapped });

        assert_round_trips(&base, SemioModelMutation::InsertSpatialNode { node: SpatialNode { id: "s2".into(), kind: SpatialKind::Building, name: "Bldg".into(), parent_id: Some("s1".into()), placement: sample_transform() } });
        assert_round_trips(&base, SemioModelMutation::RemoveSpatialNode { id: "s1".into() });
        assert_round_trips(&base, SemioModelMutation::SetSpatialNode { id: "s1".into(), kind: Some(SpatialKind::Storey), name: Some("Renamed".into()), parent_id: Some(None), placement: Some(sample_transform()) });

        assert_round_trips(&base, SemioModelMutation::InsertElement { element: SemioModelElement { id: "e2".into(), class: ElementClass::Door, placement: sample_transform(), geometry: GeometryRef::Mesh { mesh_id: "m1".into() }, spatial_id: Some("s1".into()), psets: vec![] } });
        assert_round_trips(&base, SemioModelMutation::RemoveElement { id: "e1".into() });
        assert_round_trips(&base, SemioModelMutation::SetElement { id: "e1".into(), class: Some(ElementClass::Column), placement: Some(sample_transform()), geometry: Some(GeometryRef::Brep { brep_id: "b1".into() }), spatial_id: Some(Some("s1".into())), psets: Some(vec![]) });

        assert_round_trips(&base, SemioModelMutation::InsertRelation { relation: ModelRelation { id: "r2".into(), kind: RelationKind::VoidsElement, from: "e1".into(), to: "s1".into() } });
        assert_round_trips(&base, SemioModelMutation::RemoveRelation { id: "r1".into() });
        assert_round_trips(&base, SemioModelMutation::SetRelation { id: "r1".into(), kind: Some(RelationKind::FillsVoid), from: Some("e1".into()), to: Some("s1".into()) });
    }

    /// 🧪️ op_text_binary_roundtrip_law: handcrafted `OpText`/`OpBinary` JSON round-trip, one
    /// instance of every variant.
    #[test]
    fn op_text_binary_roundtrip_law() {
        let base = base_snapshot();
        let variants = vec![
            SemioModelMutation::NoMutation,
            SemioModelMutation::SetSnapshot { snapshot: base.clone() },
            SemioModelMutation::InsertSpatialNode { node: SpatialNode { id: "s2".into(), kind: SpatialKind::Space, name: "Room".into(), parent_id: None, placement: SemioTransform::identity() } },
            SemioModelMutation::RemoveSpatialNode { id: "s1".into() },
            SemioModelMutation::SetSpatialNode { id: "s1".into(), kind: Some(SpatialKind::Storey), name: None, parent_id: Some(Some("root".into())), placement: None },
            SemioModelMutation::InsertElement { element: SemioModelElement { id: "e2".into(), class: ElementClass::Beam, placement: SemioTransform::identity(), geometry: GeometryRef::None, spatial_id: None, psets: vec![] } },
            SemioModelMutation::RemoveElement { id: "e1".into() },
            SemioModelMutation::SetElement { id: "e1".into(), class: None, placement: None, geometry: Some(GeometryRef::None), spatial_id: Some(None), psets: None },
            SemioModelMutation::InsertRelation { relation: ModelRelation { id: "r2".into(), kind: RelationKind::Other { label: "custom".into() }, from: "e1".into(), to: "s1".into() } },
            SemioModelMutation::RemoveRelation { id: "r1".into() },
            SemioModelMutation::SetRelation { id: "r1".into(), kind: Some(RelationKind::ConnectsTo), from: None, to: None },
        ];
        for m in variants {
            let printed = m.print_op();
            assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
            let parsed = SemioModelMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
            assert_eq!(parsed, m, "print_op/parse_op round-trip mismatch for {m:?}");

            let encoded = m.encode_op().unwrap_or_else(|e| panic!("encode_op({m:?}) failed: {e}"));
            let decoded = SemioModelMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
            assert_eq!(decoded, m, "encode_op/decode_op round-trip mismatch for {m:?}");
        }
    }
}
//#endregion 🔖️Tests
