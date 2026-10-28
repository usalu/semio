//! ↩️ Original authored PV thermal field inverse.

use crate::snapshot::reference::EnergyModelSnapshot;
pub fn inverse<Op: From<super::ChangePvSystemFaimanHeatLossConstant>>(payload:&super::ChangePvSystemFaimanHeatLossConstant,base:&EnergyModelSnapshot)->Result<Vec<Op>,semio_framework_value::ValueError>{
    Ok(match base.model.pv_systems.iter().find(|item|item.id==payload.id){
        Some(item) if item.faiman_heat_loss_constant_w_m2k!=payload.new_faiman_heat_loss_constant_w_m2k && !(!payload.new_faiman_heat_loss_constant_w_m2k.is_finite() || payload.new_faiman_heat_loss_constant_w_m2k <= 0.0)=>vec![crate::change_pv_system_faiman_heat_loss_constant(payload.id,item.faiman_heat_loss_constant_w_m2k)],
        _=>Vec::new(),
    })
}
