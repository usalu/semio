//! 🔺️ A setpoint kind replaces its actual complete original tagged control law.
use crate::{diff::{ReferenceEnergyModelDiff as EnergyModelDiff,ModelPatch,SetpointManagerPatch},diff_patch::Rows,snapshot::reference::EnergyModelSnapshot};
pub fn diff(p:&super::ReplaceSetpointManagerKind,b:&EnergyModelSnapshot)->protocol::MutationOutcome<EnergyModelDiff>{
 let Some(row)=b.model.setpoint_managers.iter().find(|r|r.id==p.id)else{return protocol::MutationOutcome::error("mutation.target-missing","The addressed setpoint manager does not exist.",[p.id.0.to_string()]);};
 let Some(kind)=crate::model::SetpointManagerKind::from_parts(p.new_kind,[p.new_low_outdoor_c,p.new_high_outdoor_c,p.new_low_setpoint_c,p.new_high_setpoint_c])else{return protocol::MutationOutcome::fatal("mutation.invariant","Only a finite ordered outdoor reset carries nonzero limits.",[p.id.0.to_string()]);};
 if row.kind==kind{return protocol::MutationOutcome::empty().warning("mutation.no-op","This setpoint manager already runs the control law.");}
 protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch{setpoint_managers:Rows::modifying(SetpointManagerPatch{kind:Some(kind),..SetpointManagerPatch::of(p.id)}),..Default::default()}))
}
