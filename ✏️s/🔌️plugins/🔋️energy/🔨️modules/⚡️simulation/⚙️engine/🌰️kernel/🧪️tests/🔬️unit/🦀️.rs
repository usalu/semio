use super::*;
use crate::precompute::PrecomputedModel;

#[path = "../../../⚡️electrical/🧪️tests/🔆️pv-plane/🦀️.rs"]
mod solar_pv;
use crate::schedule::ScheduleId;
use semio_s_plugin_energy_engine_model::ZoneEquipmentType;
use semio_s_plugin_energy_engine_model::SurfaceClass;
use semio_s_plugin_energy_engine_model::FaultType;

#[test]
fn initialize_creates_zone_states() {
    let model = crate::model::test_model_single_zone();
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
    let mut model = crate::model::test_model_single_zone();
    model.mechanical_ventilations.push(MechanicalVentilation { id: EntityId(90), zone_id: EntityId(1), schedule_id: ScheduleId(0), design_flow_m3_s: 0.05, fan_total_efficiency: 0.6, fan_delta_pressure_pa: 500.0 });
    model.zone_equipment.push(ZoneEquipmentAssignment { id: EntityId(91), zone_id: EntityId(1), equipment_type: ZoneEquipmentType::FanCoil, priority: 1, heating_capacity_w: 3000.0, cooling_capacity_w: 3000.0 });
    let pre = PrecomputedModel::build(&model, 60, 60);
    let weather = default_weather(10);
    let mut state = SimulationKernel::initialize(&model, &pre, &weather);
    let date = SimDate::new(2026, 1, 1);
    let config = SimulationConfig::default();
    let result = SimulationKernel::advance_timestep(&model, &config, &pre, &mut state, &weather, &date, 10.0);
    assert!(result.is_ok());
    assert!(state.zones.contains_key(&EntityId(1)));
}

#[test]
fn advance_timestep_with_baseboard_zone_equipment_and_humidistat() {
    use crate::model::*;
    let mut model = crate::model::test_model_single_zone();
    model.zone_equipment.push(ZoneEquipmentAssignment { id: EntityId(92), zone_id: EntityId(1), equipment_type: ZoneEquipmentType::Baseboard, priority: 1, heating_capacity_w: 2000.0, cooling_capacity_w: 0.0 });
    model.humidistats.push(Humidistat { id: EntityId(93), zone_id: EntityId(1), humidifying_setpoint_schedule_id: ScheduleId(0), dehumidifying_setpoint_schedule_id: ScheduleId(0), humidifying_throttle_range: 5.0, dehumidifying_throttle_range: 5.0 });
    let pre = PrecomputedModel::build(&model, 60, 60);
    let weather = default_weather(10);
    let mut state = SimulationKernel::initialize(&model, &pre, &weather);
    let date = SimDate::new(2026, 1, 1);
    let config = SimulationConfig::default();
    let result = SimulationKernel::advance_timestep(&model, &config, &pre, &mut state, &weather, &date, 10.0);
    assert!(result.is_ok());
    let zs = state.zones.get(&EntityId(1)).unwrap();
    assert!(zs.delivered.heating_w >= 0.0);
}

#[test]
fn advance_timestep_handles_ground_and_adiabatic_surfaces() {
    use crate::model::*;
    let mut model = crate::model::test_model_single_zone();
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
    let result = SimulationKernel::advance_timestep(&model, &config, &pre, &mut state, &weather, &date, 10.0);
    assert!(result.is_ok());
}

#[test]
fn advance_timestep_with_airflow_network() {
    use crate::model::*;
    let mut model = crate::model::test_model_single_zone();
    model.airflow_network = Some(AirflowNetworkDefinition { zone_node_ids: vec![(EntityId(1), 1)], outdoor_node_id: 0, link_ids: vec![] });
    let pre = PrecomputedModel::build(&model, 60, 60);
    let weather = default_weather(10);
    let mut state = SimulationKernel::initialize(&model, &pre, &weather);
    let date = SimDate::new(2026, 1, 1);
    let config = SimulationConfig::default();
    let result = SimulationKernel::advance_timestep(&model, &config, &pre, &mut state, &weather, &date, 10.0);
    assert!(result.is_ok());
}

#[test]
fn weekly_schedule_lookup_reads_sunday_first_slots() {
    use crate::schedule::ScheduleId;
    use crate::schedule::{DailySchedule, ScheduleContext, ScheduleInterpolation, ScheduleSet, WeeklySchedule};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🗓️schedule/🧪️tests/🧫️fixtures/weekly-day-slots/🔣️.json")).unwrap();
    let hour = fixture["hour"].as_u64().unwrap() as u8;
    let mut daily = Vec::new();
    let mut ids = [ScheduleId(0); 7];
    for (slot, value) in fixture["dailyValueBySlot"].as_array().unwrap().iter().enumerate() {
        let id = ScheduleId((slot as u32) + 1);
        ids[slot] = id;
        daily.push(DailySchedule { id, hourly_values: [value.as_f64().unwrap(); 24], interpolation: ScheduleInterpolation::Discrete, limits: None });
    }
    let set = ScheduleSet { daily, weekly: vec![WeeklySchedule { id: ScheduleId(100), daily_schedule_ids: ids }], ..Default::default() };
    for case in fixture["cases"].as_array().unwrap() {
        let dow = case["dayOfWeek"].as_u64().unwrap() as u8;
        let ctx = ScheduleContext { year: 2026, month: 1, day: 1, hour, day_of_week: dow, timestep_index: 0, is_dst: false };
        let mut work = None;
        let mut got = None;
        for _ in 0..64 {
            if let Some(value) = super::schedule_lookup_step(&mut work, &set, ScheduleId(100), &ctx) {
                got = Some(value);
                break;
            }
        }
        assert!((got.expect("weekly lookup") - case["value"].as_f64().unwrap()).abs() < 1e-9);
    }
}

#[test]
fn advance_timestep_applies_fault_severity_to_ideal_loads() {
    use crate::model::*;
    let mut model = crate::model::test_model_single_zone();
    model.faults.push(FaultDefinition { id: EntityId(94), target_equipment_id: EntityId(40), fault_type: FaultType::CoilFouling, severity: 0.3, start_schedule_id: ScheduleId(0) });
    let pre = PrecomputedModel::build(&model, 60, 60);
    let weather = default_weather(10);
    let mut state = SimulationKernel::initialize(&model, &pre, &weather);
    let date = SimDate::new(2026, 1, 1);
    let config = SimulationConfig::default();
    let result = SimulationKernel::advance_timestep(&model, &config, &pre, &mut state, &weather, &date, 10.0);
    assert!(result.is_ok());
}

#[test]
fn retained_thermal_free_constrained_and_partial_delivery_match_independent_balance() {
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🌡️thermal/🔣️.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap(){
  let model=crate::model::test_model_single_zone();let mut pre=PrecomputedModel::build(&model,60,60);let zone_id=pre.zone_order[0];
  pre.enclosures.get_mut(&zone_id).unwrap().faces.truncate(1);
  let weather=default_weather(0);let mut state=SimulationKernel::initialize(&model,&pre,&weather);let mut step=TimestepWork::new(&model,&pre,HourWeather {previous:weather,current:weather,next:weather},SimDate::new(2026,1,1),0.0);
  let air=&state.zones.get(&zone_id).unwrap().air;let (storage,history)=air.storage_terms(air.capacity_rate_w_k(model.zones[0].volume_m3*model.zones[0].multiplier.max(1) as f64,step.step_weather.pressure_pa,step.dt_s));
  let target=row["target"].as_f64();let loads=ZoneLoads {convective_w:440.0-history,controlled:target.is_some(),heating_setpoint_c:target.filter(|x|*x>22.0).unwrap_or(-100.0),cooling_setpoint_c:target.filter(|x|*x<22.0).unwrap_or(100.0),..Default::default()};step.zone_loads[0]=loads;
  let mut balance=BalanceWork::new();balance.stage=BalanceStage::Solve;balance.air_diagonal_w_k=23.0-storage;state.solver.matrix[..4].copy_from_slice(&[13.0,-3.0,-3.0,0.0]);state.solver.rhs[0]=180.0;step.balance=Some(balance);step.stage=TimestepStage::Balance;
  let pointers=(state.solver.matrix.as_ptr(),state.solver.scratch.as_ptr(),state.solver.solution.as_ptr());let mut turns=0;
  while step.balance.as_ref().unwrap().stage==BalanceStage::Solve {
   let saved=serde_json::to_string(&step).unwrap();let signature=step.retained_wire_signature();assert_eq!(signature.len(),fixture["checkpoint"]["words"].as_u64().unwrap() as usize);let original=state.solver.scratch.clone();
   let denied=step.step_funded(&model,&SimulationConfig::default(),&pre,&mut state,semio_framework_value::RetainedCloneGrant {maximum_items:0,maximum_copy_bytes:512,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:1});assert_eq!(denied.progress,Default::default());assert!(denied.fault.is_none());assert_eq!(serde_json::to_string(&step).unwrap(),saved);assert_eq!(state.solver.scratch,original);assert_eq!(step.retained_wire_signature(),signature);
   let turn=step.step_funded(&model,&SimulationConfig::default(),&pre,&mut state,semio_framework_value::RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:512,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:1});assert!(turn.fault.is_none(),"{:?}",turn.fault);assert_eq!(turn.progress.retained_capacity_bytes,0);assert!(turn.progress.copied_bytes<=512);turns+=1;assert!(turns<1000);assert_ne!(step.retained_wire_signature(),signature,"every funded thermal prefix has distinct continuation authority");
   let restored:TimestepWork=serde_json::from_str(&serde_json::to_string(&step).unwrap()).unwrap();assert_eq!(restored.retained_wire_signature(),step.retained_wire_signature());step=restored;
  }
  let balance=step.balance.as_mut().unwrap();assert!((balance.free_temp_c-row["free"][1].as_f64().unwrap()).abs()<1e-10);assert!((balance.required_w-row["required"].as_f64().unwrap()).abs()<1e-10);balance.delivered_w=row["delivered"].as_f64().unwrap();balance.stage=BalanceStage::Settle;
  while step.balance.as_ref().unwrap().stage==BalanceStage::Settle {let turn=step.step_funded(&model,&SimulationConfig::default(),&pre,&mut state,semio_framework_value::RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:512,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:1});assert!(turn.fault.is_none());turns+=1;assert!(turns<2000);}
  assert!((state.solver.solution[0]-row["settled"][0].as_f64().unwrap()).abs()<1e-10);assert!((state.solver.solution[1]-row["settled"][1].as_f64().unwrap()).abs()<1e-10);assert_eq!(pointers,(state.solver.matrix.as_ptr(),state.solver.scratch.as_ptr(),state.solver.solution.as_ptr()));
  println!("[DEBUG] original retained Energy thermal {} matched Mathjs free/constrained/delivered solution in {} turns",row["id"],turns);
 }
}

#[test]
fn retained_thermal_fault_refuses_physical_commit_and_restored_foreign_dimensions() {
 let model=crate::model::test_model_single_zone();let mut pre=PrecomputedModel::build(&model,60,60);let zone_id=pre.zone_order[0];pre.enclosures.get_mut(&zone_id).unwrap().faces.truncate(1);let weather=default_weather(0);
 for foreign in [false,true] {
  let mut state=SimulationKernel::initialize(&model,&pre,&weather);let original=state.zones.get(&zone_id).unwrap().air.clone();let mut step=TimestepWork::new(&model,&pre,HourWeather {previous:weather,current:weather,next:weather},SimDate::new(2026,1,1),0.0);
  let mut balance=BalanceWork::new();balance.stage=BalanceStage::Solve;balance.thermal_phase=2;balance.dense=Some(if foreign {DenseSolveCursor::new(3,9,3)}else{DenseSolveCursor::new(2,4,2)});step.balance=Some(balance);step.stage=TimestepStage::Balance;state.solver.scratch[..4].copy_from_slice(&[1.0,2.0,2.0,4.0]);state.solver.solution[..2].copy_from_slice(&[3.0,6.0]);
  let mut turns=0;
  loop {let turn=step.step_funded(&model,&SimulationConfig::default(),&pre,&mut state,semio_framework_value::RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:512,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:1});if let Some(fault)=turn.fault{assert_eq!(fault,if foreign {DenseSolveFault::Dimensions}else{DenseSolveFault::Singular});assert!(turn.progress.copied_bytes<=512);break;}turns+=1;assert!(turns<100);}
  assert_eq!(state.zones.get(&zone_id).unwrap().air,original);assert_eq!(step.balance.as_ref().unwrap().stage,BalanceStage::Solve);
  println!("[DEBUG] retained Energy typed {} fault stopped before original physical commit",if foreign {"foreign dimension"}else{"singular"});
 }
}

#[test]
fn unfunded_nonthermal_timestep_does_not_advance_and_funded_step_reports_progress() {
    let model = crate::model::test_model_single_zone();
    let pre = PrecomputedModel::build(&model, 60, 60);
    let weather = default_weather(0);
    let mut state = SimulationKernel::initialize(&model, &pre, &weather);
    let mut step = TimestepWork::new(&model, &pre, HourWeather { previous: weather, current: weather, next: weather }, SimDate::new(2026, 1, 1), 0.0);
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/timestep-grants.json")).unwrap();
    let config = SimulationConfig::default();
    for row in fixture["refused"].as_array().unwrap() {
        let saved = serde_json::to_string(&step).unwrap();
        let turn = step.step_funded(&model, &config, &pre, &mut state, RetainedCloneGrant {
            maximum_items: row["items"].as_u64().unwrap() as usize,
            maximum_copy_bytes: 512, maximum_capacity_bytes: 0, maximum_release_bytes: 0,
            maximum_depth: row["depth"].as_u64().unwrap() as usize,
        });
        assert_eq!(turn.progress, RetainedCloneProgress::default());
        assert!(turn.fault.is_none());
        assert_eq!(serde_json::to_string(&step).unwrap(), saved);
    }
    let turn = step.step_funded(&model, &config, &pre, &mut state, RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 512, maximum_capacity_bytes: 0, maximum_release_bytes: 0, maximum_depth: 1 });
    assert_eq!(turn.progress.copied_items, fixture["funded_progress_items"].as_u64().unwrap() as usize);
    assert!(turn.fault.is_none());
    step.set_stage_for_gate(TimestepStage::Complete);
    let turn = step.step_funded(&model, &config, &pre, &mut state, RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 512, maximum_capacity_bytes: 0, maximum_release_bytes: 0, maximum_depth: 1 });
    assert_eq!(turn.progress, RetainedCloneProgress::default());
}
