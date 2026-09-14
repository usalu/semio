//! 🌤️ Site, weather, EPW ingest, design days, solar position, ground temperatures.
//!
//! 🔗 EPW text decoding is delegated in-process to stdio's real, lossless `stdio.epw` artifact
//! codec (`semio_s_artifact_stdio_epw::standards::energyplus::subsets::any::io::decode_epw`,
//! all 35 spec columns, no silent defaults) — see [`EpwWeather::parse`]/[`EpwWeather::from_snapshot`].
//! Energy's own [`WeatherRecord`]/psychrometrics stay energy-side, computed FROM stdio's
//! `EpwSnapshot` rather than populated by an ad-hoc energy-side parse.

use crate::error::Error;
use crate::props::{humidity_ratio_from_rh, moist_air_density};
use crate::units::{deg_to_rad, rad_to_deg};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};
use semio_s_artifact_stdio_epw::standards::energyplus::subsets::any::schema::snapshot::EpwRecord;
use semio_s_artifact_stdio_epw::EpwSnapshot;
use serde::{Deserialize, Serialize};

// #region 🔖️WeatherRecord
/// 🌡️ One timestep of outdoor weather.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct WeatherRecord {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub dry_bulb_c: f64,
    pub dew_point_c: f64,
    pub relative_humidity: f64,
    pub atmospheric_pressure_pa: f64,
    pub wind_speed_m_s: f64,
    pub wind_direction_deg: f64,
    pub direct_normal_irradiance_w_m2: f64,
    pub diffuse_horizontal_irradiance_w_m2: f64,
    pub horizontal_infrared_w_m2: f64,
    pub precipitation_mm: f64,
    pub snow_depth_mm: f64,
}

impl WeatherRecord {
    pub fn humidity_ratio(&self) -> f64 {
        humidity_ratio_from_rh(self.dry_bulb_c, self.relative_humidity, self.atmospheric_pressure_pa)
    }

    pub fn air_density(&self) -> f64 {
        moist_air_density(self.dry_bulb_c, self.humidity_ratio(), self.atmospheric_pressure_pa)
    }
}

/// 🔢️ Parses one EPW wire field (always a `String` in stdio's lossless `EpwRecord`) into a
/// numeric type. A hard error on malformed content — no `unwrap_or` silent defaulting.
fn parse_epw_field<T: std::str::FromStr>(value: &str, field: &str) -> Result<T, Error> {
    value.trim().parse::<T>().map_err(|_| Error::fatal(format!("EPW: invalid numeric value for {field}: {value:?}")))
}

impl TryFrom<&EpwRecord> for WeatherRecord {
    type Error = Error;

    /// 🔁️ Derives energy's own per-timestep view from one of stdio's fully-labeled, 35-column
    /// `EpwRecord`s (https://bigladdersoftware.com/epx/docs/9-6/auxiliary-programs/energyplus-weather-file-epw-data-dictionary.html#field-list-locations-of-the-data-in-the-epw-file).
    /// EPW's `hour` column is 1..24 (hour-ending); converted here to a 0..23 index.
    fn try_from(r: &EpwRecord) -> Result<Self, Error> {
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
}
// #endregion 🔖️WeatherRecord

// #region 🔖️Epw
/// 📄️ EPW weather file parsed into typed records.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct EpwWeather {
    pub location: String,
    pub latitude_deg: f64,
    pub longitude_deg: f64,
    pub elevation_m: f64,
    pub time_zone_hours: f64,
    pub records: Vec<WeatherRecord>,
}

impl EpwWeather {
    /// 📥️ Parse EPW text content (EnergyPlus Weather format) via stdio's real, lossless
    /// `stdio.epw` codec (all 8 header lines + all 35 record columns, hard errors on malformed
    /// input — no silent per-field defaulting), then derive energy's own `WeatherRecord` view.
    pub fn parse(content: &str) -> Result<Self, Error> {
        let snapshot = semio_s_artifact_stdio_epw::standards::energyplus::subsets::any::io::decode_epw(content).map_err(Error::fatal)?;
        Self::from_snapshot(&snapshot)
    }

    /// 🔁️ Builds energy's derived weather view from stdio's already-decoded, lossless
    /// `EpwSnapshot` (e.g. when the snapshot was obtained via `io_dispatch`/`io_compose_via`
    /// rather than from raw text).
    pub fn from_snapshot(snapshot: &EpwSnapshot) -> Result<Self, Error> {
        let latitude_deg = parse_epw_field(&snapshot.location.latitude, "LOCATION.latitude")?;
        let longitude_deg = parse_epw_field(&snapshot.location.longitude, "LOCATION.longitude")?;
        let time_zone_hours = parse_epw_field(&snapshot.location.time_zone, "LOCATION.timeZone")?;
        let elevation_m = parse_epw_field(&snapshot.location.elevation, "LOCATION.elevation")?;
        let location = snapshot.location.city.clone();

        let records = snapshot.records.iter().map(WeatherRecord::try_from).collect::<Result<Vec<_>, Error>>()?;
        if records.is_empty() {
            return Err(Error::fatal("EPW: no data records"));
        }

        Ok(Self { location, latitude_deg, longitude_deg, elevation_m, time_zone_hours, records })
    }

    pub fn record_at_index(&self, idx: usize) -> Option<&WeatherRecord> {
        self.records.get(idx)
    }
}
// #endregion 🔖️Epw

// #region 🔖️DesignDay
/// 🌡️ Sizing design day specification.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub enum DesignDayKind {
    Heating,
    Cooling,
    Custom,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct DesignDay {
    pub name: String,
    pub kind: DesignDayKind,
    pub month: u8,
    pub day: u8,
    pub dry_bulb_max_c: f64,
    pub daily_range_k: f64,
    pub humidity_condition: DesignDayHumidity,
    pub wind_speed_m_s: f64,
    pub solar_model: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub enum DesignDayHumidity {
    Wetbulb { wetbulb_at_max_c: f64 },
    Dewpoint { dewpoint_c: f64 },
    RelativeHumidity { rh: f64 },
}

impl DesignDay {
    pub fn hourly_dry_bulb(&self, hour: u8) -> f64 {
        let h = hour as f64;
        let min_t = self.dry_bulb_max_c - self.daily_range_k;
        if h < 6.0 || h > 18.0 {
            min_t
        } else {
            let phase = (h - 6.0) / 12.0 * std::f64::consts::PI;
            min_t + self.daily_range_k * phase.sin()
        }
    }
}
// #endregion 🔖️DesignDay

// #region 🔖️Solar
/// ☀️ Solar position for a site and datetime.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct SolarPosition {
    pub altitude_deg: f64,
    pub azimuth_deg: f64,
    pub equation_of_time_min: f64,
}

const SINE_DECLINATION_COEFFICIENTS: [f64; 9] = [0.00561800, 0.0657911, -0.392779, 0.00064440, -0.00618495, -0.00010101, -0.00007951, -0.00011691, 0.00002096];
const EQUATION_OF_TIME_COEFFICIENTS: [f64; 9] = [0.00021971, -0.122649, 0.00762856, -0.156308, -0.0530028, -0.00388702, -0.00123978, -0.00270502, -0.00167992];

/// 📆️ Daily sine of the solar declination and equation of time [h] from the BLAST/EnergyPlus
/// nine-term Fourier series in `0.017167 · day_of_year`.
///
/// See the EnergyPlus Engineering Reference, "Solar Position".
pub fn daily_solar_coefficients(day_of_year: u16) -> (f64, f64) {
    let x = 0.017167 * day_of_year as f64;
    let (sine, cosine) = (x.sin(), x.cos());
    let cosine_2 = cosine * cosine - sine * sine;
    let sine_2 = 2.0 * sine * cosine;
    let terms = [1.0, sine, cosine, sine_2, cosine_2, sine * cosine_2 + cosine * sine_2, cosine * cosine_2 - sine * sine_2, 2.0 * sine_2 * cosine_2, cosine_2 * cosine_2 - sine_2 * sine_2];
    let fold = |coefficients: &[f64; 9]| coefficients.iter().zip(terms).map(|(coefficient, term)| coefficient * term).sum::<f64>();
    (fold(&SINE_DECLINATION_COEFFICIENTS), fold(&EQUATION_OF_TIME_COEFFICIENTS))
}

/// 🧭️ Unit vector towards the sun (`x` east, `y` north, `z` up) at `hour_of_day` local standard
/// time, where `hour_of_day` is the END of the timestep (0 = midnight).
pub fn sun_direction(latitude_deg: f64, longitude_deg: f64, time_zone_hours: f64, day_of_year: u16, hour_of_day: f64) -> [f64; 3] {
    let (sine_declination, equation_of_time_h) = daily_solar_coefficients(day_of_year);
    let cosine_declination = (1.0 - sine_declination * sine_declination).sqrt();
    let hour_angle = deg_to_rad(15.0 * (12.0 - (hour_of_day + equation_of_time_h)) + (time_zone_hours * 15.0 - longitude_deg));
    let latitude = deg_to_rad(latitude_deg);
    [
        cosine_declination * hour_angle.sin(),
        sine_declination * latitude.cos() - cosine_declination * latitude.sin() * hour_angle.cos(),
        sine_declination * latitude.sin() + cosine_declination * latitude.cos() * hour_angle.cos(),
    ]
}

/// ☀️ Solar altitude and azimuth (clockwise from north) at `hour_of_day` local standard time.
pub fn solar_position(latitude_deg: f64, longitude_deg: f64, time_zone_hours: f64, day_of_year: u16, hour_of_day: f64) -> SolarPosition {
    let direction = sun_direction(latitude_deg, longitude_deg, time_zone_hours, day_of_year, hour_of_day);
    let altitude_deg = rad_to_deg(direction[2].clamp(-1.0, 1.0).asin());
    let azimuth_deg = rad_to_deg(direction[0].atan2(direction[1])).rem_euclid(360.0);
    SolarPosition { altitude_deg, azimuth_deg, equation_of_time_min: daily_solar_coefficients(day_of_year).1 * 60.0 }
}

/// 🌡️ Sky temperature [°C] from the horizontal infrared irradiance, `(IR/σ)^¼`.
pub fn sky_temperature_c(horizontal_infrared_w_m2: f64) -> f64 {
    (horizontal_infrared_w_m2.max(0.0) / crate::units::STEFAN_BOLTZMANN).powf(0.25) - 273.15
}
// #endregion 🔖️Solar

// #region 🔖️TimestepWeather
/// 🌦️ Weather at the end of one zone timestep inside an hour, interpolated the way EnergyPlus
/// reads an hourly weather file: state variables linearly from the previous hour's record to the
/// current one, solar irradiance centred on the half hour and interpolated towards the next hour.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct TimestepWeather {
    pub dry_bulb_c: f64,
    pub dew_point_c: f64,
    pub humidity_ratio: f64,
    pub wet_bulb_c: f64,
    pub pressure_pa: f64,
    pub wind_speed_m_s: f64,
    pub wind_direction_deg: f64,
    pub beam_normal_w_m2: f64,
    pub diffuse_horizontal_w_m2: f64,
    pub sky_temperature_c: f64,
    pub raining: bool,
}

/// 🌧️ Hourly liquid precipitation [mm] at or above which a timestep counts as raining.
pub const RAIN_THRESHOLD_MM: f64 = 0.8;

impl TimestepWeather {
    /// 🌦️ Interpolates timestep `step` of `steps` (1-based, end of step) inside the hour of
    /// `current`, given the hour before it and the next hour whose irradiance closes the interval.
    pub fn interpolate(previous: &WeatherRecord, current: &WeatherRecord, next: &WeatherRecord, step: u32, steps: u32) -> Self {
        let weight = step as f64 / steps.max(1) as f64;
        let linear = |before: f64, now: f64| before * (1.0 - weight) + now * weight;
        let solar = |before: f64, now: f64, after: f64| {
            if steps <= 1 {
                now
            } else if weight <= 0.5 {
                before + (now - before) * (weight + 0.5)
            } else {
                now + (after - now) * (weight - 0.5)
            }
        };
        let precipitation = |record: &WeatherRecord| if record.precipitation_mm >= 999.0 { 0.0 } else { record.precipitation_mm.max(0.0) };
        let dry_bulb_c = linear(previous.dry_bulb_c, current.dry_bulb_c);
        let dew_point_c = linear(previous.dew_point_c, current.dew_point_c);
        let pressure_pa = linear(previous.atmospheric_pressure_pa, current.atmospheric_pressure_pa);
        let humidity_ratio = crate::props::humidity_ratio_from_dew_point(dew_point_c, pressure_pa);
        let raining = linear(precipitation(previous), precipitation(current)) >= RAIN_THRESHOLD_MM;
        let mut wind_direction_delta = (current.wind_direction_deg - previous.wind_direction_deg).rem_euclid(360.0);
        if wind_direction_delta > 180.0 {
            wind_direction_delta -= 360.0;
        }
        Self {
            dry_bulb_c,
            dew_point_c,
            humidity_ratio,
            wet_bulb_c: if raining { crate::props::wet_bulb_c(dry_bulb_c, humidity_ratio, pressure_pa) } else { dry_bulb_c },
            pressure_pa,
            wind_speed_m_s: linear(previous.wind_speed_m_s, current.wind_speed_m_s),
            wind_direction_deg: (previous.wind_direction_deg + wind_direction_delta * weight).rem_euclid(360.0),
            beam_normal_w_m2: solar(previous.direct_normal_irradiance_w_m2, current.direct_normal_irradiance_w_m2, next.direct_normal_irradiance_w_m2).max(0.0),
            diffuse_horizontal_w_m2: solar(previous.diffuse_horizontal_irradiance_w_m2, current.diffuse_horizontal_irradiance_w_m2, next.diffuse_horizontal_irradiance_w_m2).max(0.0),
            sky_temperature_c: sky_temperature_c(linear(previous.horizontal_infrared_w_m2, current.horizontal_infrared_w_m2)),
            raining,
        }
    }
}
// #endregion 🔖️TimestepWeather

// #region 🔖️Ground
/// 🌍️ Ground temperature model.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub enum GroundTemperatureModel {
    Monthly { temperatures_c: [f64; 12] },
    Shallow { annual_amplitude_k: f64, phase_shift_days: f64, mean_c: f64 },
    Deep { temperature_c: f64 },
}

impl GroundTemperatureModel {
    pub fn temperature_c(&self, day_of_year: u16) -> f64 {
        match self {
            Self::Monthly { temperatures_c } => {
                let month = ((day_of_year as f64 - 1.0) / 30.44) as usize % 12;
                temperatures_c[month]
            }
            Self::Shallow { annual_amplitude_k, phase_shift_days, mean_c } => {
                let phase = 2.0 * std::f64::consts::PI * (day_of_year as f64 - phase_shift_days) / 365.0;
                mean_c + annual_amplitude_k * phase.cos()
            }
            Self::Deep { temperature_c } => *temperature_c,
        }
    }
}

/// 🚰️ Water mains temperature model.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub enum WaterMainsModel {
    Constant { temperature_c: f64 },
    Monthly { temperatures_c: [f64; 12] },
}
// #endregion 🔖️Ground

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
