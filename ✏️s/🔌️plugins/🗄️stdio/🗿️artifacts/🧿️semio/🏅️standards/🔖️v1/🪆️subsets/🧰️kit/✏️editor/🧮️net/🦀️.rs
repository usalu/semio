//! 🧮️ Net of one snapshot edit as kit domain leaves: the delta between the edited snapshot and the base, expressed as the
//! concrete `kit` mutations that carry the base to it. A change the vocabulary cannot express yields leaves whose fold differs from
//! the edit, which the editor's publication check refuses.

use crate::standards::v1::subsets::base::schema::triples::{net_keyed, net_ordered, NetStep};
use crate::standards::v1::subsets::kit::schema::mutations::{add_design, add_type, bind_representation, change_representation_pin, create_model, create_object, create_properties, delete_model, delete_object, delete_properties, edit_design, remove_design, remove_type, rename_type, unbind_representation, SemioKitMutation};
use crate::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn net(base: &SemioKitSnapshot, next: &SemioKitSnapshot) -> Vec<SemioKitMutation> {
    let types = net_keyed(&base.types, &next.types, |item| item.id.clone());
    let designs = net_keyed(&base.designs, &next.designs, |item| item.id.clone());
    let objects = net_keyed(&base.objects, &next.objects, |item| item.child_id.clone());
    let models = net_keyed(&base.models, &next.models, |item| item.child_id.clone());
    let mut out = Vec::new();
    out.extend(designs.removed.iter().map(|design| SemioKitMutation::RemoveDesign(remove_design::RemoveDesign { id: design.id.clone() })));
    for step in net_ordered(&base.representations, &next.representations) {
        match step {
            NetStep::Modify { index, item } if base.representations[index].target == item.target && base.representations[index].role == item.role => {
                out.push(SemioKitMutation::ChangeRepresentationPin(change_representation_pin::ChangeRepresentationPin { index, pin: item.pin.clone() }));
            }
            NetStep::Modify { index, item } => {
                out.push(SemioKitMutation::UnbindRepresentation(unbind_representation::UnbindRepresentation { index }));
                out.push(SemioKitMutation::BindRepresentation(bind_representation::BindRepresentation { target: item.target.clone(), pin: item.pin.clone(), role: item.role.clone(), at: None }));
            }
            NetStep::Remove { index } => out.push(SemioKitMutation::UnbindRepresentation(unbind_representation::UnbindRepresentation { index })),
            NetStep::Insert { index, item } => out.push(SemioKitMutation::BindRepresentation(bind_representation::BindRepresentation { target: item.target.clone(), pin: item.pin.clone(), role: item.role.clone(), at: Some(index) })),
        }
    }
    out.extend(types.removed.iter().map(|item| SemioKitMutation::RemoveType(remove_type::RemoveType { id: item.id.clone() })));
    out.extend(types.modified.iter().filter(|(before, after)| before.name != after.name).map(|(_, after)| SemioKitMutation::RenameType(rename_type::RenameType { id: after.id.clone(), new_name: after.name.clone() })));
    out.extend(types.added.iter().map(|item| SemioKitMutation::AddType(add_type::AddType { id: item.id.clone(), name: item.name.clone(), category: item.category.clone(), at: None })));
    for design in &designs.added {
        out.push(SemioKitMutation::AddDesign(add_design::AddDesign { id: design.id.clone(), name: design.name.clone(), at: None }));
        if !design.pieces.is_empty() || !design.connections.is_empty() {
            out.push(SemioKitMutation::EditDesign(edit_design::EditDesign { id: design.id.clone(), pieces: design.pieces.clone(), connections: design.connections.clone() }));
        }
    }
    out.extend(designs.modified.iter().filter(|(before, after)| before.pieces != after.pieces || before.connections != after.connections).map(|(_, after)| SemioKitMutation::EditDesign(edit_design::EditDesign { id: after.id.clone(), pieces: after.pieces.clone(), connections: after.connections.clone() })));
    out.extend(objects.removed.iter().map(|item| SemioKitMutation::DeleteObject(delete_object::DeleteObject { child_id: item.child_id.clone() })));
    out.extend(objects.added.iter().map(|item| SemioKitMutation::CreateObject(create_object::CreateObject { child_id: item.child_id.clone(), target: item.target.clone(), at: None })));
    out.extend(models.removed.iter().map(|item| SemioKitMutation::DeleteModel(delete_model::DeleteModel { child_id: item.child_id.clone() })));
    out.extend(models.added.iter().map(|item| SemioKitMutation::CreateModel(create_model::CreateModel { child_id: item.child_id.clone(), target: item.target.clone(), at: None })));
    match (&base.properties, &next.properties) {
        (Some(before), Some(after)) if before != after => {
            out.push(SemioKitMutation::DeleteProperties(delete_properties::DeleteProperties {}));
            out.push(SemioKitMutation::CreateProperties(create_properties::CreateProperties { child_id: after.child_id.clone(), target: after.target.clone() }));
        }
        (Some(_), None) => out.push(SemioKitMutation::DeleteProperties(delete_properties::DeleteProperties {})),
        (None, Some(after)) => out.push(SemioKitMutation::CreateProperties(create_properties::CreateProperties { child_id: after.child_id.clone(), target: after.target.clone() })),
        _ => {}
    }
    out
}
