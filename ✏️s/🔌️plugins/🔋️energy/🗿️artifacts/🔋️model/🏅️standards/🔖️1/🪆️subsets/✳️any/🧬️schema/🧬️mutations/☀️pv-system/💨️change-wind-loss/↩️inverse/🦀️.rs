//! ↩️ Original authored PV thermal field inverse.

use crate::snapshot::reference::EnergyModelSnapshot;
pub fn inverse<Op: From<super::ChangePvSystemFaimanHeatLossWind>>(payload:&super::ChangePvSystemFaimanHeatLossWind,base:&EnergyModelSnapshot)->Result<Vec<Op>,semio_framework_value::ValueError>{
    Ok(match base.model.pv_systems.iter().find(|item|item.id==payload.id){
        Some(item) if item.faiman_heat_loss_wind_w_s_m3k!=payload.new_faiman_heat_loss_wind_w_s_m3k && !(!payload.new_faiman_heat_loss_wind_w_s_m3k.is_finite() || payload.new_faiman_heat_loss_wind_w_s_m3k < 0.0)=>vec![crate::change_pv_system_faiman_heat_loss_wind(payload.id,item.faiman_heat_loss_wind_w_s_m3k)],
        _=>Vec::new(),
    })
}
