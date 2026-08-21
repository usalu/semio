//! 🌤️ Site, weather, EPW ingest, design days, solar position, ground temperatures.
//!
//! 🔗 EPW text decoding is delegated in-process to stdio's real, lossless `stdio.epw` artifact
//! codec (`semio_s_plugin_stdio::artifacts::epw::standards::energyplus::subsets::any::io::decode_epw`,
//! all 35 spec columns, no silent defaults) — see [`EpwWeather::parse`]/[`EpwWeather::from_snapshot`].
//! Energy's own [`WeatherRecord`]/psychrometrics stay energy-side, computed FROM stdio's
//! `EpwSnapshot` rather than populated by an ad-hoc energy-side parse.

use crate::error::Error;
use crate::props::{humidity_ratio_from_rh, moist_air_density};
use crate::units::{deg_to_rad, rad_to_deg};
use semio_s_plugin_stdio::artifacts::epw::standards::energyplus::subsets::any::schema::snapshot::EpwRecord;
use semio_s_plugin_stdio::artifacts::epw::EpwSnapshot;
use serde::{Deserialize, Serialize};

// #region 🔖️WeatherRecord
/// 🌡️ One timestep of outdoor weather.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
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
    pub async fn humidity_ratio(&self) -> f64 {
        humidity_ratio_from_rh(self.dry_bulb_c, self.relative_humidity, self.atmospheric_pressure_pa)
    }

    pub async fn air_density(&self) -> f64 {
        moist_air_density(self.dry_bulb_c, self.humidity_ratio(), self.atmospheric_pressure_pa)
    }
}

/// 🔢️ Parses one EPW wire field (always a `String` in stdio's lossless `EpwRecord`) into a
/// numeric type. A hard error on malformed content — no `unwrap_or` silent defaulting.
async fn parse_epw_field<T: std::str::FromStr>(value: &str, field: &str) -> Result<T, Error> {
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
    pub async fn parse(content: &str) -> Result<Self, Error> {
        let snapshot = semio_s_plugin_stdio::artifacts::epw::standards::energyplus::subsets::any::io::decode_epw(content).map_err(Error::fatal)?;
        Self::from_snapshot(&snapshot)
    }

    /// 🔁️ Builds energy's derived weather view from stdio's already-decoded, lossless
    /// `EpwSnapshot` (e.g. when the snapshot was obtained via `io_dispatch`/`io_compose_via`
    /// rather than from raw text).
    pub async fn from_snapshot(snapshot: &EpwSnapshot) -> Result<Self, Error> {
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

    pub async fn record_at_index(&self, idx: usize) -> Option<&WeatherRecord> {
        self.records.get(idx)
    }

    /// 📈️ Interpolate weather to sub-hourly timestep.
    pub async fn interpolate(&self, hour_index: f64) -> WeatherRecord {
        let idx = hour_index.floor() as usize;
        let frac = hour_index - idx as f64;
        let a = self.records.get(idx).copied().unwrap_or_else(|| self.records[0]);
        let b = self.records.get(idx + 1).copied().unwrap_or(a);
        WeatherRecord {
            dry_bulb_c: a.dry_bulb_c + frac * (b.dry_bulb_c - a.dry_bulb_c),
            dew_point_c: a.dew_point_c + frac * (b.dew_point_c - a.dew_point_c),
            relative_humidity: a.relative_humidity + frac * (b.relative_humidity - a.relative_humidity),
            wind_speed_m_s: a.wind_speed_m_s + frac * (b.wind_speed_m_s - a.wind_speed_m_s),
            direct_normal_irradiance_w_m2: a.direct_normal_irradiance_w_m2 + frac * (b.direct_normal_irradiance_w_m2 - a.direct_normal_irradiance_w_m2),
            diffuse_horizontal_irradiance_w_m2: a.diffuse_horizontal_irradiance_w_m2 + frac * (b.diffuse_horizontal_irradiance_w_m2 - a.diffuse_horizontal_irradiance_w_m2),
            ..a
        }
    }
}
// #endregion 🔖️Epw

// #region 🔖️DesignDay
/// 🌡️ Sizing design day specification.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DesignDayKind {
    Heating,
    Cooling,
    Custom,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DesignDayHumidity {
    Wetbulb { wetbulb_at_max_c: f64 },
    Dewpoint { dewpoint_c: f64 },
    RelativeHumidity { rh: f64 },
}

impl DesignDay {
    pub async fn hourly_dry_bulb(&self, hour: u8) -> f64 {
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
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct SolarPosition {
    pub altitude_deg: f64,
    pub azimuth_deg: f64,
    pub equation_of_time_min: f64,
}

/// ☀️ Compute solar altitude and azimuth (simplified SPA).
pub async fn solar_position(latitude_deg: f64, longitude_deg: f64, day_of_year: u16, hour_solar: f64) -> SolarPosition {
    let lat = deg_to_rad(latitude_deg);
    let decl = deg_to_rad(23.45 * (360.0 * (day_of_year as f64 - 81.0) / 365.0).to_radians().sin());
    let ha = deg_to_rad(15.0 * (hour_solar - 12.0));
    let sin_alt = lat.sin() * decl.sin() + lat.cos() * decl.cos() * ha.cos();
    let altitude_deg = rad_to_deg(sin_alt.clamp(-1.0, 1.0).asin());
    let cos_az = (decl.sin() - lat.sin() * sin_alt) / (lat.cos() * sin_alt.clamp(0.001, 1.0).acos().cos().max(1e-6));
    let azimuth_deg = rad_to_deg(cos_az.clamp(-1.0, 1.0).acos());
    let equation_of_time_min = 4.0 * (longitude_deg - 15.0 * (hour_solar / 24.0 * 24.0).round());
    SolarPosition { altitude_deg, azimuth_deg, equation_of_time_min }
}

/// 🌡️ Sky temperature [K] from dry-bulb and dew-point (Brunt-type).
pub async fn sky_temperature_k(t_dry_c: f64, t_dew_c: f64) -> f64 {
    let t_dry_k = t_dry_c + 273.15;
    let emissivity = 0.711 + 0.0056 * t_dew_c + 0.000_073 * t_dew_c * t_dew_c + 0.013 * (t_dew_c * 0.1).cos();
    t_dry_k * emissivity.powf(0.25)
}
// #endregion 🔖️Solar

// #region 🔖️Ground
/// 🌍️ Ground temperature model.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum GroundTemperatureModel {
    Monthly { temperatures_c: [f64; 12] },
    Shallow { annual_amplitude_k: f64, phase_shift_days: f64, mean_c: f64 },
    Deep { temperature_c: f64 },
}

impl GroundTemperatureModel {
    pub async fn temperature_c(&self, day_of_year: u16) -> f64 {
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum WaterMainsModel {
    Constant { temperature_c: f64 },
    Monthly { temperatures_c: [f64; 12] },
}
// #endregion 🔖️Ground

#[cfg(test)]
mod tests {
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

    #[semio_framework_async_macros::async_test]
    async fn epw_parses_minimal() {
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
    #[semio_framework_async_macros::async_test]
    async fn weather_record_derives_correct_fields_from_stdio_snapshot() {
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

    #[semio_framework_async_macros::async_test]
    async fn epw_parse_rejects_malformed_text() {
        assert!(EpwWeather::parse("not an epw file").is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn solar_noon_altitude_positive() {
        let pos = solar_position(45.0, 0.0, 172, 12.0);
        assert!(pos.altitude_deg > 0.0);
    }
}
