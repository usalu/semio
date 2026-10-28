//! 🌦️ EPW-specific conversion into format-independent simulation weather.

extern crate semio_s_plugin_energy_engine_bestest as bestest;
extern crate semio_s_plugin_energy_engine_error as error;
extern crate semio_s_plugin_energy_engine_results as results;
extern crate semio_s_plugin_energy_engine_site as site;
#[cfg(test)]
extern crate semio_s_plugin_energy_engine_sim as sim;

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
    if !(1..=24).contains(&hour_1_24) { return Err(Error::fatal("EPW: hour-ending source hour must be in1..24.")); }
    let relative_humidity: f64 = parse_epw_field::<f64>(&r.relative_humidity, "relativeHumidity")? / 100.0;
    Ok(WeatherRecord {
        year: parse_epw_field(&r.year, "year")?,
        month: parse_epw_field(&r.month, "month")?,
        day: parse_epw_field(&r.day, "day")?,
        hour: hour_1_24 - 1,
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
        snow_depth_mm: parse_epw_field::<f64>(&r.snow_depth, "snowDepth")? * 10.0,
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
    if snapshot.records.is_empty() { return Err(Error::fatal("EPW: no data records")); }
    let metadata = weather_metadata(snapshot)?;
    let records = snapshot.records.iter().map(weather_record).collect::<Result<Vec<_>, Error>>()?;
    Ok(WeatherData { calendar: metadata.calendar, location: metadata.location.to_owned(), latitude_deg: metadata.latitude_deg, longitude_deg: metadata.longitude_deg, elevation_m: metadata.elevation_m, time_zone_hours: metadata.time_zone_hours, records })
}

/// 🧭️ Borrows declared EPW location and calendar independently of per-record numerical projection.
pub struct WeatherMetadata<'a> {
    pub calendar: crate::site::WeatherCalendar,
    pub location: &'a str,
    pub latitude_deg: f64,
    pub longitude_deg: f64,
    pub elevation_m: f64,
    pub time_zone_hours: f64,
}

/// 🗓️ Validates the shared weather header without cloning or traversing its hourly records.
pub fn weather_metadata(snapshot: &EpwSnapshot) -> Result<WeatherMetadata<'_>, Error> {
    if snapshot.data_periods.records_per_hour != 1 { return Err(Error::fatal("EPW: simulation weather requires one hourly source record.")); }
    let mut dated = None;
    for period in &snapshot.data_periods.periods {
        let start=period.start_date.split('/').count()==3;let end=period.end_date.split('/').count()==3;
        if start!=end||dated.is_some_and(|dated|dated!=start){return Err(Error::fatal("EPW: DATA PERIODS mixes actual-date and typical-year bounds."));}
        dated=Some(start);
    }
    let calendar=match dated{Some(true)=>crate::site::WeatherCalendar::ActualDates,Some(false)=>crate::site::WeatherCalendar::TypicalYear,None=>return Err(Error::fatal("EPW: DATA PERIODS has no authored calendar bounds."))};
    Ok(WeatherMetadata {
        calendar,
        location: &snapshot.location.city,
        latitude_deg: parse_epw_field(&snapshot.location.latitude, "LOCATION.latitude")?,
        longitude_deg: parse_epw_field(&snapshot.location.longitude, "LOCATION.longitude")?,
        elevation_m: parse_epw_field(&snapshot.location.elevation, "LOCATION.elevation")?,
        time_zone_hours: parse_epw_field(&snapshot.location.time_zone, "LOCATION.timeZone")?,
    })
}

/// 🫴️ Parsing faults retain their diagnostic while admitted weather failures retain the complete original owner.
#[derive(Debug)]
pub enum BestestJobFailure{Parse(Diagnostics),Admission(crate::bestest::BestestJobFailure)}

/// 🏛️ Prepares a caller-owned BESTEST job from its authored EPW input.
pub fn bestest_job(operation:crate::bestest::Operation,case: &str, epw_text: &str, warmup_days: u32) -> Result<crate::bestest::EnergyJob, BestestJobFailure> {
    let weather = parse(epw_text).map_err(|error| {
        let mut diagnostics = Diagnostics::default();
        diagnostics.push(error);
        BestestJobFailure::Parse(diagnostics)
    })?;
    crate::bestest::simulation_job(operation,case,weather,warmup_days).map_err(BestestJobFailure::Admission)
}

/// 📊 Projects already completed results; simulation remains with its original caller.
pub fn results_report_json(case:&str,results:&crate::results::Results,epw_file:&str,epw_sha256:&str)->String{crate::bestest::results_report_json(case,results,epw_file,epw_sha256)}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
