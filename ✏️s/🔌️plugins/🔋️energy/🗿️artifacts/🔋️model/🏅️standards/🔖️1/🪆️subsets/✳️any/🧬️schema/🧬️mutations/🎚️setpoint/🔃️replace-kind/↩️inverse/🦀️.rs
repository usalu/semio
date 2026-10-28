//! ↩️ The inverse preserves the original control law as copied discriminator and four fixed limits.
use crate::snapshot::reference::EnergyModelSnapshot;
pub fn inverse<Op:From<super::ReplaceSetpointManagerKind>>(p:&super::ReplaceSetpointManagerKind,b:&EnergyModelSnapshot)->Result<Vec<Op>,semio_framework_value::ValueError>{
 let Some(kind)=crate::model::SetpointManagerKind::from_parts(p.new_kind,[p.new_low_outdoor_c,p.new_high_outdoor_c,p.new_low_setpoint_c,p.new_high_setpoint_c])else{return Ok(Vec::new());};
 Ok(match b.model.setpoint_managers.iter().find(|r|r.id==p.id){Some(r)if r.kind!=kind=>{let(tag,[low,high,low_setpoint,high_setpoint])=r.kind.parts();vec![super::ReplaceSetpointManagerKind{id:p.id,new_kind:tag,new_low_outdoor_c:low,new_high_outdoor_c:high,new_low_setpoint_c:low_setpoint,new_high_setpoint_c:high_setpoint}.into()]},_=>Vec::new()})
}
