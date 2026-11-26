//! 🧫️ Shared model and weather inputs for numerical admission tests.

use semio_s_plugin_energy_engine_kernel::SimulationConfig;
use semio_s_plugin_energy_engine_model::{EntityId, Model, Zone};
use semio_s_plugin_energy_engine_site::{design_day_hour, WeatherCalendar, WeatherData};

pub fn inputs(fixture: &serde_json::Value) -> (Model, SimulationConfig) {
    let mut model = Model::default();
    model.zones.reserve_exact(fixture["zoneCapacity"].as_u64().unwrap() as usize);
    for (name, id) in fixture["zones"].as_array().unwrap().iter().zip(1..) {
        model.zones.push(Zone {
            id: EntityId(id),
            name: name.as_str().unwrap().into(),
            volume_m3: 40.0,
            multiplier: 1,
            conditioned: true,
            part_of_total_floor_area: true,
        });
    }
    let mut records = Vec::with_capacity(fixture["weatherCapacity"].as_u64().unwrap() as usize);
    for hour in 0..fixture["weatherRecords"].as_u64().unwrap() {
        records.push(design_day_hour(hour as u32, -10.0));
    }
    let config = SimulationConfig {
        weather: Some(WeatherData {
            calendar: WeatherCalendar::ActualDates,
            location: "Original".into(),
            latitude_deg: 0.0,
            longitude_deg: 0.0,
            elevation_m: 0.0,
            time_zone_hours: 0.0,
            records,
        }),
        ..Default::default()
    };
    (model, config)
}
