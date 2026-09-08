
use super::*;
use crate::precompute::PrecomputedModel;

#[test]
fn initialize_creates_zone_states() {
    let model = crate::sim::test_model_single_zone();
    let pre = PrecomputedModel::build(&model, 60, 60);
    let weather = default_weather(0);
    let state = SimulationKernel::initialize(&model, &pre, &weather);
    assert!(state.zones.contains_key(&EntityId(1)));
}

#[test]
fn energy_balance_near_zero_for_steady_state() {
    let residual = SimulationKernel::energy_balance_check(1000.0, 200.0, 800.0);
    assert!(residual < 1e-6);
}

#[test]
fn run_period_from_config() {
    let config = SimulationConfig { run_period_start_month: 1, run_period_start_day: 1, run_period_end_month: 1, run_period_end_day: 7, ..Default::default() };
    assert_eq!(SimulationKernel::run_period(&config).total_hours(), 168);
}

#[test]
fn advance_timestep_with_mechanical_ventilation_and_fan_coil_zone_equipment() {
    use crate::model::*;
    let mut model = crate::sim::test_model_single_zone();
    model.mechanical_ventilations.push(MechanicalVentilation { id: EntityId(90), zone_id: EntityId(1), schedule_id: ScheduleId(0), design_flow_m3_s: 0.05, fan_total_efficiency: 0.6, fan_delta_pressure_pa: 500.0 });
    model.zone_equipment.push(ZoneEquipmentAssignment { id: EntityId(91), zone_id: EntityId(1), equipment_type: ZoneEquipmentType::FanCoil, priority: 1, heating_capacity_w: 3000.0, cooling_capacity_w: 3000.0 });
    let pre = PrecomputedModel::build(&model, 60, 60);
    let weather = default_weather(10);
    let mut state = SimulationKernel::initialize(&model, &pre, &weather);
    let date = SimDate::new(2026, 1, 1);
    let config = SimulationConfig::default();
    let result = SimulationKernel::advance_timestep(&model, &config, &pre, &mut state, &weather, &date, 10.0, pre.zone_timestep_s);
    assert!(result.is_ok());
    assert!(state.zones.contains_key(&EntityId(1)));
}

#[test]
fn advance_timestep_with_baseboard_zone_equipment_and_humidistat() {
    use crate::model::*;
    let mut model = crate::sim::test_model_single_zone();
    model.zone_equipment.push(ZoneEquipmentAssignment { id: EntityId(92), zone_id: EntityId(1), equipment_type: ZoneEquipmentType::Baseboard, priority: 1, heating_capacity_w: 2000.0, cooling_capacity_w: 0.0 });
    model.humidistats.push(Humidistat { id: EntityId(93), zone_id: EntityId(1), humidifying_setpoint_schedule_id: ScheduleId(0), dehumidifying_setpoint_schedule_id: ScheduleId(0), humidifying_throttle_range: 5.0, dehumidifying_throttle_range: 5.0 });
    let pre = PrecomputedModel::build(&model, 60, 60);
    let weather = default_weather(10);
    let mut state = SimulationKernel::initialize(&model, &pre, &weather);
    let date = SimDate::new(2026, 1, 1);
    let config = SimulationConfig::default();
    let result = SimulationKernel::advance_timestep(&model, &config, &pre, &mut state, &weather, &date, 10.0, pre.zone_timestep_s);
    assert!(result.is_ok());
    let zs = state.zones.get(&EntityId(1)).unwrap();
    assert!(zs.delivered.heating_w >= 0.0);
}

#[test]
fn advance_timestep_handles_ground_and_adiabatic_surfaces() {
    use crate::model::*;
    let mut model = crate::sim::test_model_single_zone();
    model.surfaces[0].outside_boundary_condition = OutsideBoundary::Ground;
    model.surfaces.push(Surface {
        id: EntityId(31),
        name: "AdiabaticWall".into(),
        zone_id: EntityId(1),
        class: SurfaceClass::InteriorWall,
        vertices_m: vec![[0.0, 0.0, 0.0], [5.0, 0.0, 0.0], [5.0, 0.0, 3.0], [0.0, 0.0, 3.0]],
        construction_id: EntityId(20),
        outside_boundary_condition: OutsideBoundary::Adiabatic,
        sun_exposed: false,
        wind_exposed: false,
        multiplier: 1,
    });
    let pre = PrecomputedModel::build(&model, 60, 60);
    let weather = default_weather(10);
    let mut state = SimulationKernel::initialize(&model, &pre, &weather);
    let date = SimDate::new(2026, 1, 1);
    let config = SimulationConfig::default();
    let result = SimulationKernel::advance_timestep(&model, &config, &pre, &mut state, &weather, &date, 10.0, pre.zone_timestep_s);
    assert!(result.is_ok());
}

#[test]
fn advance_timestep_with_airflow_network() {
    use crate::model::*;
    let mut model = crate::sim::test_model_single_zone();
    model.airflow_network = Some(AirflowNetworkDefinition { zone_node_ids: vec![(EntityId(1), 1)], outdoor_node_id: 0, link_ids: vec![] });
    let pre = PrecomputedModel::build(&model, 60, 60);
    let weather = default_weather(10);
    let mut state = SimulationKernel::initialize(&model, &pre, &weather);
    let date = SimDate::new(2026, 1, 1);
    let config = SimulationConfig::default();
    let result = SimulationKernel::advance_timestep(&model, &config, &pre, &mut state, &weather, &date, 10.0, pre.zone_timestep_s);
    assert!(result.is_ok());
}

#[test]
fn advance_timestep_applies_fault_severity_to_ideal_loads() {
    use crate::model::*;
    let mut model = crate::sim::test_model_single_zone();
    model.faults.push(FaultDefinition { id: EntityId(94), target_equipment_id: EntityId(40), fault_type: FaultType::CoilFouling, severity: 0.3, start_schedule_id: ScheduleId(0) });
    let pre = PrecomputedModel::build(&model, 60, 60);
    let weather = default_weather(10);
    let mut state = SimulationKernel::initialize(&model, &pre, &weather);
    let date = SimDate::new(2026, 1, 1);
    let config = SimulationConfig::default();
    let result = SimulationKernel::advance_timestep(&model, &config, &pre, &mut state, &weather, &date, 10.0, pre.zone_timestep_s);
    assert!(result.is_ok());
}
