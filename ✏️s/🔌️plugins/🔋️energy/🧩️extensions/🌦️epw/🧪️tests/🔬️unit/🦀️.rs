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
    let vectors: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🌦️weather/🔣️.json")).unwrap();
    for row in vectors["cases"].as_array().unwrap() {
        let input = row["input"].as_str().unwrap();
        let actual = parse(input);
        let mut reader = csv::ReaderBuilder::new().has_headers(false).flexible(true).from_reader(input.as_bytes());
        let oracle_rows = reader.records().collect::<Result<Vec<_>, _>>();
        let oracle_accepted = oracle_rows.as_ref().is_ok_and(|rows| {
            rows.len() > 8
                && rows[0].len() == 10
                && &rows[0][0] == "LOCATION"
                && [6, 7, 8, 9].iter().all(|index| rows[0][*index].parse::<f64>().is_ok())
                && rows[8..].iter().all(|record| record.len() == 35 && [0, 1, 2, 3, 4, 6, 7, 8, 9, 12, 14, 15, 20, 21, 30, 33].iter().all(|index| record[*index].parse::<f64>().is_ok()))
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
                let expected = serde_json::json!({ "year": number(0) as u16, "month": number(1) as u8, "day": number(2) as u8, "hour": number(3) as u8 - 1, "minute": number(4) as u8, "dry_bulb_c": number(6), "dew_point_c": number(7), "relative_humidity": number(8)/100.0, "atmospheric_pressure_pa": number(9), "wind_speed_m_s": number(21), "wind_direction_deg": number(20), "direct_normal_irradiance_w_m2": number(14), "diffuse_horizontal_irradiance_w_m2": number(15), "horizontal_infrared_w_m2": number(12), "precipitation_mm": number(33), "snow_depth_mm": number(30) });
                assert_eq!(serde_json::to_value(mapped).unwrap(), expected);
            }
            assert_eq!(weather.records.len(), records.len() - 8);
        }
    }
}
