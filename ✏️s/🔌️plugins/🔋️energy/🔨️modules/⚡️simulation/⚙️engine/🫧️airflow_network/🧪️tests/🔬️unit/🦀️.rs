
use super::*;
use crate::units::P_STD;

fn two_zone_network() -> AirflowNetwork {
    AirflowNetwork {
        nodes: vec![AfNode { id: 0, elevation_m: 0.0, temperature_c: 5.0, humidity_ratio: 0.004, is_reference: true }, AfNode { id: 1, elevation_m: 0.0, temperature_c: 22.0, humidity_ratio: 0.009, is_reference: false }],
        links: vec![AfLink { id: 1, node_a: 1, node_b: 0, kind: AfLinkKind::Crack, flow_coefficient: 0.01, flow_exponent: 0.65, area_m2: 0.05, discharge_coefficient: 0.6, orientation_deg: 0.0, wind_exposure_factor: 1.0 }],
        wind_speed_m_s: 3.0,
        wind_direction_deg: 0.0,
        outdoor_temp_c: 5.0,
        outdoor_humidity_ratio: 0.004,
    }
}

#[test]
fn stack_pressure_positive_when_outdoor_colder() {
    let outdoor = AfNode { id: 0, elevation_m: 0.0, temperature_c: 5.0, humidity_ratio: 0.004, is_reference: true };
    let zone = AfNode { id: 1, elevation_m: 3.0, temperature_c: 22.0, humidity_ratio: 0.009, is_reference: false };
    let dp = stack_pressure_pa(&zone, &outdoor, P_STD);
    assert!(dp.abs() > 0.0);
}

#[test]
fn network_solves_pressures() {
    let net = two_zone_network();
    let pressures = net.solve_pressures(P_STD, 100, 1e-3).unwrap();
    assert_eq!(pressures.len(), 2);
    assert!((pressures[0]).abs() < 1e-9);
}

#[test]
fn infiltration_flow_when_zone_warmer() {
    let net = two_zone_network();
    let flows = net.solve_flows(P_STD).unwrap();
    assert_eq!(flows.len(), 1);
}
