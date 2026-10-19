use super::*;
use crate::kernel::HourWeather;
use semio_framework_job_payload::JobPayloadStream;
use std::time::Instant;
use crate::model::test_model_single_zone;

#[test]
fn p7c2_lossless_queue_saturation_retains_identity_and_fifo_order() {
    let mut queue = EnergyWireQueue::new(EnergyWireKind::Checkpoint);
    for sequence in 0..ENERGY_WIRE_QUEUE_SLOTS as u64 {
        let identity = EnergyWireIdentity { operation: 1, base_revision: 2, generation: 3, seed: 4, sequence };
        queue
            .push(EnergyWirePacket { kind: EnergyWireKind::Checkpoint, identity, payload: RetainedJobPayload::empty(JobPayloadStream::CheckpointState), preview: None, reservation: None })
            .expect("exact queue maximum");
    }
    let overflow_identity = EnergyWireIdentity { operation: 1, base_revision: 2, generation: 3, seed: 4, sequence: ENERGY_WIRE_QUEUE_SLOTS as u64 };
    let rejected = queue
        .push(EnergyWirePacket { kind: EnergyWireKind::Checkpoint, identity: overflow_identity, payload: RetainedJobPayload::empty(JobPayloadStream::CheckpointState), preview: None, reservation: None })
        .expect_err("queue MAX+1 retains packet");
    assert_eq!(rejected.identity, overflow_identity);
    let first = queue.take().expect("first lease");
    assert_eq!(first.identity().sequence, 0);
    queue.retry(first).expect("retry exact head");
    for sequence in 0..ENERGY_WIRE_QUEUE_SLOTS as u64 {
        let lease = queue.take().expect("FIFO lease");
        assert_eq!(lease.identity().sequence, sequence);
        queue.ack(lease).expect("empty packet ACK");
    }
}

#[test]
fn p7c2_lossless_queue_drop_and_panic_recover_exact_in_flight_head() {
    let identity = EnergyWireIdentity { operation: 11, base_revision: 12, generation: 13, seed: 14, sequence: 15 };
    let mut queue = EnergyWireQueue::new(EnergyWireKind::Checkpoint);
    queue
        .push(EnergyWirePacket { kind: EnergyWireKind::Checkpoint, identity, payload: RetainedJobPayload::empty(JobPayloadStream::CheckpointState), preview: None, reservation: None })
        .expect("fixed queue accepts head");
    drop(queue.take().expect("drop lease"));
    let recovered = queue.take().expect("Drop republishes exact in-flight head");
    assert_eq!(recovered.identity(), identity);
    queue.retry(recovered).expect("retry returns exact head to original slot");
    let panic_lease = queue.take().expect("panic lease");
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
        let _owned = panic_lease;
        panic!("hostile consumer panic");
    }));
    let recovered = queue.take().expect("panic Drop republishes exact head");
    assert_eq!(recovered.identity(), identity);
    queue.ack(recovered).expect("empty recovered packet ACKs once");
    assert_eq!(queue.len, 0);
}

#[test]
fn adversarial_timestep_work_unit_stays_below_watchdog() {
    let mut model = test_model_single_zone();
    let template = model.surfaces[0].clone();
    model.surfaces = (0..16_384)
        .map(|index| {
            let mut surface = template.clone();
            surface.id = model::EntityId(1_000 + index);
            surface
        })
        .collect();
    let pre = PrecomputedModel::build(&model, 60, 15);
    let weather = site::design_day_hour(12, 35.0);
    let date = calendar::SimDate::new(weather.year, weather.month, weather.day);
    let mut state = SimulationKernel::initialize(&model, &pre, &weather);
    let mut work = TimestepWork::new(&model, &pre, HourWeather { previous: weather, current: weather, next: weather }, date, 12.0);
    let start = Instant::now();
    work.step(&model, &SimulationConfig::default(), &pre, &mut state);
    assert!(start.elapsed() < std::time::Duration::from_millis(8), "one adversarial energy work unit exceeded watchdog: {:?}", start.elapsed());
}

#[test]
fn p7c1_every_independent_maximum_plus_one_dimension_rejects() {
    let census = EnergyNumericalCensus {
        zones: 1,
        surfaces: 1,
        fenestrations: 1,
        people: 1,
        lighting: 1,
        equipment: 1,
        infiltrations: 1,
        airflow_nodes: 1,
        airflow_links: 1,
        mechanical_ventilations: 1,
        thermostats: 1,
        humidistats: 1,
        ideal_loads: 1,
        faults: 1,
        zone_equipment: 1,
        plant_loops: 1,
        plant_equipment: 1,
        pv_systems: 1,
        batteries: 1,
        service_hot_water: 1,
        refrigeration: 1,
        water: 1,
        weather_records: 1,
        timesteps: 1,
        meters: 1,
        series: 1,
        samples: 1,
        history_values: 1,
        summary_rows: 1,
        identifier_bytes: 1,
        observed_items: 1,
        observed_bytes: 1,
        pages: 1,
        operations: 1,
        process_jobs: 1,
    };
    macro_rules! rejects {
        ($field:ident, $variant:ident) => {{
            let mut maximum = census;
            maximum.$field = 0;
            assert_eq!(census.first_exceeded(maximum), Some(EnergyNumericalDimension::$variant));
        }};
    }
    rejects!(zones, Zones);
    rejects!(surfaces, Surfaces);
    rejects!(fenestrations, Fenestrations);
    rejects!(people, People);
    rejects!(lighting, Lighting);
    rejects!(equipment, Equipment);
    rejects!(infiltrations, Infiltrations);
    rejects!(airflow_nodes, AirflowNodes);
    rejects!(airflow_links, AirflowLinks);
    rejects!(mechanical_ventilations, MechanicalVentilations);
    rejects!(thermostats, Thermostats);
    rejects!(humidistats, Humidistats);
    rejects!(ideal_loads, IdealLoads);
    rejects!(faults, Faults);
    rejects!(zone_equipment, ZoneEquipment);
    rejects!(plant_loops, PlantLoops);
    rejects!(plant_equipment, PlantEquipment);
    rejects!(pv_systems, PvSystems);
    rejects!(batteries, Batteries);
    rejects!(service_hot_water, ServiceHotWater);
    rejects!(refrigeration, Refrigeration);
    rejects!(water, Water);
    rejects!(weather_records, WeatherRecords);
    rejects!(timesteps, Timesteps);
    rejects!(meters, Meters);
    rejects!(series, Series);
    rejects!(samples, Samples);
    rejects!(history_values, HistoryValues);
    rejects!(summary_rows, SummaryRows);
    rejects!(identifier_bytes, IdentifierBytes);
    rejects!(observed_items, ObservedItems);
    rejects!(observed_bytes, ObservedBytes);
    rejects!(pages, Pages);
    rejects!(operations, Operations);
    rejects!(process_jobs, ProcessJobs);
}

#[test]
fn p7c1_language_agnostic_law_fixture_matches_reference_parser() {
    let source = include_str!("../../../../../../../🧪️tests/🧫️fixtures/🧮️numerical-laws/🔣️.json");
    let reference: semio_framework_pack_json::Value = semio_framework_pack_json::parse(source, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("reference JSON parser");
    let marker = "\"schema\": \"";
    let start = source.find(marker).expect("schema field") + marker.len();
    let end = start + source[start..].find('"').expect("schema terminator");
    let schema = &source[start..end];
    assert_eq!(reference["schema"].as_str(), Some(schema));
    assert_eq!(reference["step"]["fuel"].as_u64(), Some(1));
}
