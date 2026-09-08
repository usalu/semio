
use super::*;

#[test]
fn flat_plate_gain_positive_at_noon() {
    let collector = FlatPlateCollector { area_m2: 10.0, tau_alpha: 0.75, ul_w_m2k: 3.5, iam_factor: 0.95 };
    let gain = collector.useful_gain_w(800.0, 20.0, 2.0, 25.0, 0.2, 4180.0);
    assert!(gain > 0.0);
}

#[test]
fn zero_irradiance_zero_gain() {
    let q = collector_thermal_output_w(CollectorKind::FlatPlate, 5.0, 0.0, 15.0, 1.0, 20.0, 0.1, 4180.0, 0.7, 4.0, 1.0, 20.0);
    assert!(q.abs() < 1.0);
}

#[test]
fn ics_raises_storage_temperature() {
    let ics = IntegralCollectorStorage { area_m2: 3.0, storage_volume_l: 200.0, tau_alpha: 0.8, loss_coefficient_w_m2k: 5.0 };
    let (new_t, gain) = ics.simulate(25.0, 700.0, 18.0, 3600.0);
    assert!(gain > 0.0);
    assert!(new_t > 25.0);
}

#[test]
fn unglazed_preheats_air() {
    let utc = UnglazedTranspiredCollector { area_m2: 50.0, porosity: 0.6, h_conv_w_m2k: 15.0, suction_velocity_m_s: 0.04 };
    let (gain, outlet_t) = utc.preheat_air_w(600.0, 5.0, 3.0);
    assert!(gain > 0.0);
    assert!(outlet_t > 5.0);
}

#[test]
fn pvt_splits_electric_and_thermal() {
    let pvt = PvtCollector { area_m2: 8.0, pv_efficiency: 0.18, tau_alpha: 0.9, ul_w_m2k: 4.0, fluid_cp: 4180.0 };
    let (pv, thermal) = pvt.simulate(900.0, 22.0, 1.5, 30.0, 0.15);
    assert!(pv > 500.0);
    assert!(thermal >= 0.0);
}
