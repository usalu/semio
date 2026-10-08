//! 🧮️ Net of one snapshot edit as model domain leaves: the delta between the edited snapshot and the base, expressed as the
//! concrete `model` mutations that carry the base to it. A change the vocabulary cannot express yields leaves whose fold differs from
//! the edit, which the editor's publication check refuses.

use crate::standards::v1::subsets::base::schema::triples::net_keyed;
use crate::standards::v1::subsets::model::schema::mutations::{insert_element, insert_relation, insert_spatial_node, remove_element, remove_relation, remove_spatial_node, set_element, set_relation, set_spatial_node, SemioModelMutation};
use crate::standards::v1::subsets::model::schema::snapshot::SemioModelSnapshot;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn net(base: &SemioModelSnapshot, next: &SemioModelSnapshot) -> Vec<SemioModelMutation> {
    let spatial = net_keyed(&base.spatial, &next.spatial, |node| node.id.clone());
    let elements = net_keyed(&base.elements, &next.elements, |element| element.id.clone());
    let relations = net_keyed(&base.relations, &next.relations, |relation| relation.id.clone());
    let mut out = Vec::new();
    out.extend(relations.removed.iter().map(|relation| SemioModelMutation::RemoveRelation(remove_relation::RemoveRelation { id: relation.id.clone() })));
    out.extend(elements.removed.iter().map(|element| SemioModelMutation::RemoveElement(remove_element::RemoveElement { id: element.id.clone() })));
    out.extend(spatial.removed.iter().map(|node| SemioModelMutation::RemoveSpatialNode(remove_spatial_node::RemoveSpatialNode { id: node.id.clone() })));
    out.extend(spatial.added.iter().map(|node| SemioModelMutation::InsertSpatialNode(insert_spatial_node::InsertSpatialNode { node: (*node).clone(), at: None })));
    for (before, after) in &spatial.modified {
        out.push(SemioModelMutation::SetSpatialNode(set_spatial_node::SetSpatialNode {
            id: after.id.clone(),
            kind: (before.kind != after.kind).then(|| after.kind.clone()),
            name: (before.name != after.name).then(|| after.name.clone()),
            parent_id: (before.parent_id != after.parent_id).then(|| after.parent_id.clone()),
            placement: (before.placement != after.placement).then(|| after.placement.clone()),
        }));
    }
    out.extend(elements.added.iter().map(|element| SemioModelMutation::InsertElement(insert_element::InsertElement { element: (*element).clone(), at: None })));
    for (before, after) in &elements.modified {
        out.push(SemioModelMutation::SetElement(set_element::SetElement {
            id: after.id.clone(),
            class: (before.class != after.class).then(|| after.class.clone()),
            placement: (before.placement != after.placement).then(|| after.placement.clone()),
            geometry: (before.geometry != after.geometry).then(|| after.geometry.clone()),
            spatial_id: (before.spatial_id != after.spatial_id).then(|| after.spatial_id.clone()),
            psets: (before.psets != after.psets).then(|| after.psets.clone()),
        }));
    }
    out.extend(relations.added.iter().map(|relation| SemioModelMutation::InsertRelation(insert_relation::InsertRelation { relation: (*relation).clone(), at: None })));
    for (before, after) in &relations.modified {
        out.push(SemioModelMutation::SetRelation(set_relation::SetRelation {
            id: after.id.clone(),
            kind: (before.kind != after.kind).then(|| after.kind.clone()),
            from: (before.from != after.from).then(|| after.from.clone()),
            to: (before.to != after.to).then(|| after.to.clone()),
        }));
    }
    out
}
