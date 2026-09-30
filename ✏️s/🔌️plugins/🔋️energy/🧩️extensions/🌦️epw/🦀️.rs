//! 🌦️ EPW-specific conversion into format-independent simulation weather.

use crate::error::{Diagnostics, Error};
use crate::site::{WeatherData, WeatherRecord};
use semio_s_artifact_stdio_epw::standards::energyplus::subsets::any::schema::snapshot::EpwRecord;
use semio_s_artifact_stdio_epw::EpwSnapshot;

/// 🔢️ Parses one EPW wire field (always a `String` in stdio's lossless `EpwRecord`) into a
/// numeric type. A hard error on malformed content — no `unwrap_or` silent defaulting.
fn parse_epw_field<T: std::str::FromStr>(value: &str, field: &str) -> Result<T, Error> {
    value.trim().parse::<T>().map_err(|_| Error::fatal(format!("EPW: invalid numeric value for {field}: {value:?}")))
}

/// 🔁️ Derives energy's own per-timestep view from one of stdio's fully-labeled, 35-column
/// `EpwRecord`s (https://bigladdersoftware.com/epx/docs/9-6/auxiliary-programs/energyplus-weather-file-epw-data-dictionary.html#field-list-locations-of-the-data-in-the-epw-file).
/// EPW's `hour` column is 1..24 (hour-ending); converted here to a 0..23 index.
pub fn weather_record(r: &EpwRecord) -> Result<WeatherRecord, Error> {
    let hour_1_24: u8 = parse_epw_field(&r.hour, "hour")?;
    let relative_humidity: f64 = parse_epw_field::<f64>(&r.relative_humidity, "relativeHumidity")? / 100.0;
    Ok(WeatherRecord {
        year: parse_epw_field(&r.year, "year")?,
        month: parse_epw_field(&r.month, "month")?,
        day: parse_epw_field(&r.day, "day")?,
        hour: hour_1_24.saturating_sub(1),
        minute: parse_epw_field(&r.minute, "minute")?,
        dry_bulb_c: parse_epw_field(&r.dry_bulb_temp, "dryBulbTemp")?,
        dew_point_c: parse_epw_field(&r.dew_point_temp, "dewPointTemp")?,
        relative_humidity,
        atmospheric_pressure_pa: parse_epw_field(&r.atmospheric_pressure, "atmosphericPressure")?,
        wind_speed_m_s: parse_epw_field(&r.wind_speed, "windSpeed")?,
        wind_direction_deg: parse_epw_field(&r.wind_direction, "windDirection")?,
        direct_normal_irradiance_w_m2: parse_epw_field(&r.direct_normal_radiation, "directNormalRadiation")?,
        diffuse_horizontal_irradiance_w_m2: parse_epw_field(&r.diffuse_horizontal_radiation, "diffuseHorizontalRadiation")?,
        horizontal_infrared_w_m2: parse_epw_field(&r.horizontal_infrared_radiation, "horizontalInfraredRadiation")?,
        precipitation_mm: parse_epw_field(&r.liquid_precip_depth, "liquidPrecipDepth")?,
        snow_depth_mm: parse_epw_field(&r.snow_depth, "snowDepth")?,
    })
}

/// 📥️ Parse EPW text content (EnergyPlus Weather format) via stdio's real, lossless
/// `stdio.epw` codec (all 8 header lines + all 35 record columns, hard errors on malformed
/// input — no silent per-field defaulting), then derive energy's own `WeatherRecord` view.
pub fn parse(content: &str) -> Result<WeatherData, Error> {
    let snapshot = semio_s_artifact_stdio_epw::standards::energyplus::subsets::any::io::decode_epw(content).map_err(Error::fatal)?;
    from_snapshot(&snapshot)
}

/// 🔁️ Builds energy's derived weather view from stdio's already-decoded, lossless
/// `EpwSnapshot` (e.g. when the snapshot was obtained via `io_dispatch`/`io_compose_via`
/// rather than from raw text).
pub fn from_snapshot(snapshot: &EpwSnapshot) -> Result<WeatherData, Error> {
    let latitude_deg = parse_epw_field(&snapshot.location.latitude, "LOCATION.latitude")?;
    let longitude_deg = parse_epw_field(&snapshot.location.longitude, "LOCATION.longitude")?;
    let time_zone_hours = parse_epw_field(&snapshot.location.time_zone, "LOCATION.timeZone")?;
    let elevation_m = parse_epw_field(&snapshot.location.elevation, "LOCATION.elevation")?;
    let location = snapshot.location.city.clone();

    let records = snapshot.records.iter().map(weather_record).collect::<Result<Vec<_>, Error>>()?;
    if records.is_empty() {
        return Err(Error::fatal("EPW: no data records"));
    }

    Ok(WeatherData { location, latitude_deg, longitude_deg, elevation_m, time_zone_hours, records })
}

/// 🏛️ Runs a BESTEST case after decoding its authored EPW input.
pub fn run_bestest(case: &str, epw_text: &str, warmup_days: u32) -> Result<crate::results::Results, Diagnostics> {
    let weather = parse(epw_text).map_err(|error| {
        let mut diagnostics = Diagnostics::default();
        diagnostics.push(error);
        diagnostics
    })?;
    crate::bestest::run(case, weather, warmup_days)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
