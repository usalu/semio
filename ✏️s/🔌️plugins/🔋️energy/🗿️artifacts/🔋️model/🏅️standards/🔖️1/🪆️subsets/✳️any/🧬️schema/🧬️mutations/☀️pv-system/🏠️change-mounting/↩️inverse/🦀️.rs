//! ↩️ Original authored PV thermal field inverse.

use crate::snapshot::reference::EnergyModelSnapshot;
pub fn inverse<Op: From<super::ChangePvSystemMounting>>(payload:&super::ChangePvSystemMounting,base:&EnergyModelSnapshot)->Result<Vec<Op>,semio_framework_value::ValueError>{
    Ok(match base.model.pv_systems.iter().find(|item|item.id==payload.id){
        Some(item) if item.mounting!=payload.new_mounting && !(false)=>vec![crate::change_pv_system_mounting(payload.id,item.mounting)],
        _=>Vec::new(),
    })
}
