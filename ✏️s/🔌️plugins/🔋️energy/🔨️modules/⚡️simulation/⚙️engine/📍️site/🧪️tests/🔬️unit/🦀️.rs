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

#[test]
fn solar_noon_altitude_positive() {
    let pos = solar_position(45.0, 0.0, 0.0, 172, 12.0);
    assert!(pos.altitude_deg > 0.0);
}

/// 🧪️ Summer-solstice noon at 45 °N must land within a degree of the closed-form
/// `90 − latitude + declination` and point due south; the afternoon sun must swing WEST of
/// south (azimuth > 180°), which the previous `acos`-only azimuth could never express.
#[test]
fn solar_azimuth_sweeps_east_through_south_to_west() {
    let noon = solar_position(45.0, 0.0, 0.0, 172, 12.0);
    assert!((noon.altitude_deg - 68.4).abs() < 1.5, "solstice noon altitude was {}", noon.altitude_deg);
    assert!((noon.azimuth_deg - 180.0).abs() < 3.0, "solstice noon azimuth was {}", noon.azimuth_deg);
    let morning = solar_position(45.0, 0.0, 0.0, 172, 8.0);
    let afternoon = solar_position(45.0, 0.0, 0.0, 172, 16.0);
    assert!(morning.azimuth_deg < 180.0, "08:00 azimuth was {}", morning.azimuth_deg);
    assert!(afternoon.azimuth_deg > 180.0, "16:00 azimuth was {}", afternoon.azimuth_deg);
}
