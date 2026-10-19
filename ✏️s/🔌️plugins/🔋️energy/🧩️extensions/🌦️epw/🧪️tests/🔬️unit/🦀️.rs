use super::*;
use crate::bestest::{project, run};
use crate::results::Results;

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
    let w = parse(EPW_FIXTURE).unwrap();
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
    let w = parse(EPW_FIXTURE).unwrap();
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
    assert!(parse("not an epw file").is_err());
}

#[test]
fn portable_weather_vectors_match_csv_and_serde_oracles() {
    let vectors: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🌦️weather/🔣️.json")).unwrap();
    for row in vectors["cases"].as_array().unwrap() {
        let input = row["input"].as_str().unwrap();
        let actual = parse(input);
        let mut reader = csv::ReaderBuilder::new().has_headers(false).flexible(true).from_reader(input.as_bytes());
        let oracle_rows = reader.records().collect::<Result<Vec<_>, _>>();
        let oracle_accepted = oracle_rows.as_ref().is_ok_and(|rows| {
            rows.len() > 8
                && rows[0].len() == 10
                && &rows[0][0] == "LOCATION"
                && &rows[7][2] == "1"
                && rows[7][5].split('/').count().eq(&rows[7][6].split('/').count())
                && [6, 7, 8, 9].iter().all(|index| rows[0][*index].parse::<f64>().is_ok())
                && rows[8..].iter().all(|record| record.len() == 35 && record[3].parse::<u8>().is_ok_and(|hour| (1..=24).contains(&hour)) && [0, 1, 2, 3, 4, 6, 7, 8, 9, 12, 14, 15, 20, 21, 30, 33].iter().all(|index| record[*index].parse::<f64>().is_ok()))
        });
        assert_eq!(actual.is_ok(), oracle_accepted, "independent CSV admission {}", row["id"]);
        assert_eq!(actual.is_ok(), row["accepted"].as_bool().unwrap(), "{}", row["id"]);
        if let Ok(weather) = actual {
            let encoded = serde_json::to_value(&weather).unwrap();
            assert_eq!(encoded, row["expected"], "{}", row["id"]);
            let records = oracle_rows.unwrap();
            let location = &records[0];
            assert_eq!(weather.location, location[1]);
            assert_eq!(weather.latitude_deg, location[6].parse::<f64>().unwrap());
            assert_eq!(weather.longitude_deg, location[7].parse::<f64>().unwrap());
            assert_eq!(weather.time_zone_hours, location[8].parse::<f64>().unwrap());
            assert_eq!(weather.elevation_m, location[9].parse::<f64>().unwrap());
            for (mapped, wire) in weather.records.iter().zip(&records[8..]) {
                assert_eq!(wire.len(), 35);
                let number = |index: usize| wire[index].parse::<f64>().unwrap();
                let expected = serde_json::json!({ "year": number(0) as u16, "month": number(1) as u8, "day": number(2) as u8, "hour": number(3) as u8 - 1, "minute": number(4) as u8, "dry_bulb_c": number(6), "dew_point_c": number(7), "relative_humidity": number(8)/100.0, "atmospheric_pressure_pa": number(9), "wind_speed_m_s": number(21), "wind_direction_deg": number(20), "direct_normal_irradiance_w_m2": number(14), "diffuse_horizontal_irradiance_w_m2": number(15), "horizontal_infrared_w_m2": number(12), "precipitation_mm": number(33), "snow_depth_mm": number(30) * 10.0 });
                assert_eq!(serde_json::to_value(mapped).unwrap(), expected);
            }
            assert_eq!(weather.records.len(), records.len() - 8);
        }
    }
}

//#region 🌦️SyntheticWeather
/// 🌦️ A structurally real, 8760-record EPW for the annual smoke tests, built by templating the
/// committed one-day fixture's own record layout.
///
/// ⚠️ This is NOT a validation weather file and must never be used for a BESTEST comparison: the
/// temperatures are a plain annual-plus-diurnal sinusoid and the irradiance is a clear-sky
/// rectangle. It exists so the determinism, free-float and setpoint-holding laws below can run a
/// full year TODAY, before `🧫️fixtures/🌦️denver-tmy/🌦️.epw` is committed. The real comparison
/// reads that file and nothing else.
fn synthetic_annual_epw() -> String {
    const DAYS_IN_MONTH: [u16; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut text = String::new();
    text.push_str("LOCATION,Denver,Colorado,USA,semio-synthetic,724666,39.83,-104.65,-7.0,1650.0\n");
    text.push_str("DESIGN CONDITIONS,0\n");
    text.push_str("TYPICAL/EXTREME PERIODS,0\n");
    text.push_str("GROUND TEMPERATURES,0\n");
    text.push_str("HOLIDAYS/DAYLIGHT SAVINGS,No,0,0,0\n");
    text.push_str("COMMENTS 1,semio synthetic annual EPW -- structurally valid, sinusoidal, NOT a station record.\n");
    text.push_str("COMMENTS 2,Generated by crate::epw::tests::synthetic_annual_epw for the annual smoke laws.\n");
    text.push_str("DATA PERIODS,1,1,Data,Sunday, 1/ 1,12/31\n");
    let latitude = 39.83_f64.to_radians();
    let mut day_of_year = 0_u16;
    for (month_index, days) in DAYS_IN_MONTH.iter().enumerate() {
        for day in 1..=*days {
            day_of_year += 1;
            let declination = (23.45_f64 * (360.0 * (day_of_year as f64 + 284.0) / 365.0).to_radians().sin()).to_radians();
            let cos_hour_angle = (-latitude.tan() * declination.tan()).clamp(-1.0, 1.0);
            let half_day_hours = cos_hour_angle.acos().to_degrees() / 15.0;
            let seasonal = 10.0 + 12.0 * (2.0 * std::f64::consts::PI * (day_of_year as f64 - 105.0) / 365.0).sin();
            for hour in 1..=24_u32 {
                let solar_hour = hour as f64 - 0.5;
                let dry_bulb = seasonal + 8.0 * (2.0 * std::f64::consts::PI * (solar_hour - 15.0) / 24.0).cos();
                let elevation = 1.0 - ((solar_hour - 12.0) / half_day_hours.max(0.5)).abs();
                let sunlit = elevation > 0.0;
                let direct_normal = if sunlit { 900.0 * elevation } else { 0.0 };
                let diffuse = if sunlit { 110.0 * elevation } else { 0.0 };
                let global = direct_normal * elevation + diffuse;
                text.push_str(&format!(
                    "2026,{month},{day},{hour},0,?9?9?9?9E0,{dry_bulb:.1},{dew_point:.1},50,101325,0,0,300,{global:.0},{direct_normal:.0},{diffuse:.0},0,0,0,0,180,3.0,3,2,20.0,22000,0,999999999,14,0.081,0,88,0.2,0,0\n",
                    month = month_index + 1,
                    day = day,
                    hour = hour,
                    dry_bulb = dry_bulb,
                    dew_point = dry_bulb - 10.0,
                    global = global,
                    direct_normal = direct_normal,
                    diffuse = diffuse,
                ));
            }
        }
    }
    text
}

fn annual_results(case: &str) -> Results {
    run(case, parse(&synthetic_annual_epw()).expect("synthetic weather"), 3).unwrap_or_else(|diagnostics| panic!("case {case} must run: {:?}", diagnostics.messages))
}

/// 🌦️ The synthetic file must actually reach the engine through the real `🌦️epw` codec, with
/// every one of the 8760 records decoded — a silently truncated year would make every law below
/// vacuous.
#[test]
fn synthetic_annual_weather_decodes_to_a_full_year() {
    let weather = parse(&synthetic_annual_epw()).expect("the synthetic EPW parses through the stdio codec");
    assert_eq!(weather.records.len(), 8760);
}

/// 🔁️ Two runs of the same case against the same weather must agree bit for bit.
#[test]
fn annual_run_is_deterministic() {
    let first = project("600", &annual_results("600"));
    let second = project("600", &annual_results("600"));
    assert_eq!(first, second);
}

/// 🌡️ A free-float case has no HVAC at all, so it must deliver exactly zero energy while still
/// producing a full year of zone temperatures.
#[test]
fn free_float_case_delivers_no_hvac_energy() {
    let projected = project("600FF", &annual_results("600FF"));
    assert_eq!(projected.annual_heating_kwh, 0.0);
    assert_eq!(projected.annual_cooling_kwh, 0.0);
    assert_eq!(projected.zone_air_temperature_c.len(), 8760);
    assert!(projected.free_float_max_c > projected.free_float_min_c, "a free-floating zone must actually swing");
}

/// 🐛️ Free-float stability and thermal mass defects: the explicit air-only integrator swung case
/// 600FF between −118 °C and 156 °C and made 900FF identical to 600FF. The coupled implicit balance
/// must stay physical, and the heavy case must swing less and never peak above the light one.
#[test]
fn free_float_is_stable_and_mass_damps_the_swing() {
    let light = project("600FF", &annual_results("600FF"));
    let heavy = project("900FF", &annual_results("900FF"));
    for projected in [&light, &heavy] {
        assert!(projected.free_float_min_c > -40.0 && projected.free_float_max_c < 90.0, "free float left the physical range: {:.1}…{:.1} °C", projected.free_float_min_c, projected.free_float_max_c);
    }
    assert!(heavy.free_float_max_c - heavy.free_float_min_c < light.free_float_max_c - light.free_float_min_c - 5.0, "light {:.1}…{:.1} °C, heavy {:.1}…{:.1} °C", light.free_float_min_c, light.free_float_max_c, heavy.free_float_min_c, heavy.free_float_max_c);
    assert!((heavy.free_float_mean_c - light.free_float_mean_c).abs() < 3.0);
}

/// 🎛️ An ideal-loads case with unlimited capacity must hold its own setpoints: at least 99 % of
/// the year within [20 − 0.5, 27 + 0.5] °C. This is the law the predictor/corrector repair
/// exists for — the previous kernel both double-advanced the zone-air history and predicted the
/// load from a residual that omitted infiltration.
#[test]
fn controlled_case_holds_its_setpoints() {
    let projected = project("600", &annual_results("600"));
    assert_eq!(projected.zone_air_temperature_c.len(), 8760);
    let inside = projected.zone_air_temperature_c.iter().filter(|value| **value >= 19.5 && **value <= 27.5).count();
    let fraction = inside as f64 / projected.zone_air_temperature_c.len() as f64;
    assert!(fraction >= 0.99, "only {:.2} % of hours were inside the setpoints (min {:.2} °C, max {:.2} °C)", fraction * 100.0, projected.free_float_min_c, projected.free_float_max_c);
}

/// ☀️ Shading must actually do something: the overhang case cannot use more cooling than the
/// unshaded case it is derived from, and the fin case cannot use more than the overhang case.
#[test]
fn shading_never_increases_cooling() {
    let unshaded = project("600", &annual_results("600")).annual_cooling_kwh;
    let overhang = project("610", &annual_results("610")).annual_cooling_kwh;
    assert!(overhang <= unshaded + 1e-6, "case 610 cooled {overhang} kWh against case 600's {unshaded} kWh");
}
//#endregion 🌦️SyntheticWeather
