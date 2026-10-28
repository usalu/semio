//! ↩️ Original authored PV thermal field inverse.

use crate::snapshot::reference::EnergyModelSnapshot;
pub fn inverse<Op: From<super::ChangePvSystemTemperatureCoefficient>>(payload:&super::ChangePvSystemTemperatureCoefficient,base:&EnergyModelSnapshot)->Result<Vec<Op>,semio_framework_value::ValueError>{
    Ok(match base.model.pv_systems.iter().find(|item|item.id==payload.id){
        Some(item) if item.temperature_coefficient!=payload.new_temperature_coefficient && !(!payload.new_temperature_coefficient.is_finite())=>vec![crate::change_pv_system_temperature_coefficient(payload.id,item.temperature_coefficient)],
        _=>Vec::new(),
    })
}
