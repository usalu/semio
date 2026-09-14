use super::*;

/// 📄️ One real, spec-compliant 35-column EPW header+record pair (LOCATION line handcrafted;
/// data record copied verbatim from stdio's real W0 fixture
/// `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/📚️examples/🎬️demo/🖼️assets/example.epw` line 9)
/// — stdio's `decode_epw` hard-errors on anything short of exactly 35 columns, so this must
/// be genuinely well-formed, unlike the old ad-hoc parser's tolerant/truncated test fixture.
const EPW_FIXTURE: &str = "LOCATION,Hannover,Niedersachsen,DEU,semio-fixture,10238,52.37,9.74,1.0,55.0\n\
DESIGN CONDITIONS,0\n\
TYPICAL/EXTREME PERIODS,0\n\
GROUND TEMPERATURES,0\n\
HOLIDAYS/DAYLIGHT SAVINGS,0\n\
COMMENTS 1,0\n\
COMMENTS 2,0\n\
DATA PERIODS,1,1,Data,Sunday,1/1,1/1\n\
2026,1,15,1,0,?9?9?9?9E0?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9,-7.8,-12.3,92,101100,0,0,280,0,0,0,0,0,0,0,205,2.9,3,2,20.0,22000,0,999999999,14,0.081,0,88,0.2,0,0\n";

#[test]
fn epw_parses_minimal() {
    let w = EpwWeather::parse(EPW_FIXTURE).unwrap();
    assert!((w.latitude_deg - 52.37).abs() < 1e-6);
    assert!((w.longitude_deg - 9.74).abs() < 1e-6);
    assert!((w.elevation_m - 55.0).abs() < 1e-6);
    assert_eq!(w.location, "Hannover");
    assert_eq!(w.records.len(), 1);
}

/// 🐛️ Regression: the deleted ad-hoc parser read the wrong wire columns for wind
/// speed/direction and horizontal-infrared radiation (off by several columns). Deriving
/// `WeatherRecord` from stdio's labeled `EpwRecord` fields must recover the correct values.
#[test]
fn weather_record_derives_correct_fields_from_stdio_snapshot() {
    let w = EpwWeather::parse(EPW_FIXTURE).unwrap();
    let r = &w.records[0];
    assert_eq!(r.hour, 0, "EPW hour 1 (hour-ending) maps to 0-indexed hour 0");
    assert!((r.dry_bulb_c - (-7.8)).abs() < 1e-6);
    assert!((r.relative_humidity - 0.92).abs() < 1e-6, "relative humidity stored as a 0..1 fraction");
    assert!((r.wind_speed_m_s - 2.9).abs() < 1e-6, "wind speed must read EPW column 21, not 20");
    assert!((r.wind_direction_deg - 205.0).abs() < 1e-6, "wind direction must read EPW column 20, not 21");
    assert!((r.horizontal_infrared_w_m2 - 280.0).abs() < 1e-6, "horizontal infrared must read EPW column 12");
    assert!((r.precipitation_mm - 0.0).abs() < 1e-6);
    assert!((r.snow_depth_mm - 0.0).abs() < 1e-6);
}

#[test]
fn epw_parse_rejects_malformed_text() {
    assert!(EpwWeather::parse("not an epw file").is_err());
}

/// 🔮️ Sun position defect: EnergyPlus places the sun at the END of each timestep with its
/// nine-term declination and equation-of-time series. Denver TMY, 1 July, 09:10 standard time:
/// EnergyPlus 25.2 reports a solar altitude of 50.193°.
#[test]
fn sun_position_matches_energyplus_at_the_end_of_the_timestep() {
    let position = solar_position(39.83, -104.65, -7.0, 182, 9.0 + 1.0 / 6.0);
    assert!((position.altitude_deg - 50.193).abs() < 5e-3, "altitude was {}", position.altitude_deg);
    assert!(position.azimuth_deg > 90.0 && position.azimuth_deg < 180.0);
    let noon = solar_position(45.0, 0.0, 0.0, 172, 12.0);
    assert!((noon.altitude_deg - 68.4).abs() < 1.5 && (noon.azimuth_deg - 180.0).abs() < 3.0);
    assert!(solar_position(45.0, 0.0, 0.0, 172, 16.0).azimuth_deg > 180.0);
}

fn hour_record(dry_bulb_c: f64, beam_w_m2: f64, precipitation_mm: f64) -> WeatherRecord {
    let mut record = EpwWeather::parse(EPW_FIXTURE).expect("fixture").records[0];
    record.dry_bulb_c = dry_bulb_c;
    record.direct_normal_irradiance_w_m2 = beam_w_m2;
    record.precipitation_mm = precipitation_mm;
    record
}

/// 🧪️ Weather defect: temperatures interpolate linearly from the previous hour, irradiance is
/// centred on the half hour and closes towards the next hour, and rain needs 0.8 mm.
#[test]
fn timestep_weather_interpolates_like_energyplus() {
    let (previous, current, next) = (hour_record(10.0, 0.0, 0.0), hour_record(20.0, 600.0, 1.0), hour_record(30.0, 900.0, 999.0));
    let first = TimestepWeather::interpolate(&previous, &current, &next, 1, 6);
    let middle = TimestepWeather::interpolate(&previous, &current, &next, 3, 6);
    let last = TimestepWeather::interpolate(&previous, &current, &next, 6, 6);
    assert!((middle.dry_bulb_c - 15.0).abs() < 1e-9 && (last.dry_bulb_c - 20.0).abs() < 1e-9);
    assert!((first.beam_normal_w_m2 - 400.0).abs() < 1e-9 && (middle.beam_normal_w_m2 - 600.0).abs() < 1e-9 && (last.beam_normal_w_m2 - 750.0).abs() < 1e-9);
    assert!(!first.raining && last.raining && last.wet_bulb_c < last.dry_bulb_c);
    assert!(!TimestepWeather::interpolate(&current, &next, &next, 6, 6).raining, "missing precipitation (999) counts as dry");
    assert!((sky_temperature_c(crate::units::STEFAN_BOLTZMANN * 273.15_f64.powi(4))).abs() < 1e-9);
}
