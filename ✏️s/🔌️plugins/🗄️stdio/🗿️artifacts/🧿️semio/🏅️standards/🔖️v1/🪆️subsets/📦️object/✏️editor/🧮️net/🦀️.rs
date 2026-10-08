//! 🧮️ Net of one snapshot edit as object domain leaves: the delta between the edited snapshot and the base, expressed as the
//! concrete `object` mutations that carry the base to it. A change the vocabulary cannot express yields leaves whose fold differs from
//! the edit, which the editor's publication check refuses.

use crate::standards::v1::subsets::object::schema::mutations::{create_brep, create_mesh, create_properties, delete_brep, delete_mesh, delete_properties, move_object, rotate_object, scale_object, SemioObjectMutation};
use crate::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn net(base: &SemioObjectSnapshot, next: &SemioObjectSnapshot) -> Vec<SemioObjectMutation> {
    let mut out = Vec::new();
    if base.transform.translation != next.transform.translation {
        out.push(SemioObjectMutation::MoveObject(move_object::MoveObject { translation: next.transform.translation }));
    }
    if base.transform.rotation != next.transform.rotation {
        out.push(SemioObjectMutation::RotateObject(rotate_object::RotateObject { rotation: next.transform.rotation }));
    }
    if base.transform.scale != next.transform.scale {
        out.push(SemioObjectMutation::ScaleObject(scale_object::ScaleObject { scale: next.transform.scale }));
    }
    if base.brep != next.brep {
        if base.brep.is_some() {
            out.push(SemioObjectMutation::DeleteBrep(delete_brep::DeleteBrep {}));
        }
        if let Some(brep) = &next.brep {
            out.push(SemioObjectMutation::CreateBrep(create_brep::CreateBrep { child_id: brep.child_id.clone(), target: brep.target.clone() }));
        }
    }
    if base.mesh != next.mesh {
        if base.mesh.is_some() {
            out.push(SemioObjectMutation::DeleteMesh(delete_mesh::DeleteMesh {}));
        }
        if let Some(mesh) = &next.mesh {
            out.push(SemioObjectMutation::CreateMesh(create_mesh::CreateMesh { child_id: mesh.child_id.clone(), target: mesh.target.clone() }));
        }
    }
    if base.properties != next.properties {
        if base.properties.is_some() {
            out.push(SemioObjectMutation::DeleteProperties(delete_properties::DeleteProperties {}));
        }
        if let Some(properties) = &next.properties {
            out.push(SemioObjectMutation::CreateProperties(create_properties::CreateProperties { child_id: properties.child_id.clone(), target: properties.target.clone() }));
        }
    }
    out
}
