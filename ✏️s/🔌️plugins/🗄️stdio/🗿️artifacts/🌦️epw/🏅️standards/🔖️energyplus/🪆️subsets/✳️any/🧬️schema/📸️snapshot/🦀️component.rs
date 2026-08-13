//! 🧬️ EpwSnapshot schema — persistent fields; real EnergyPlus Weather codec lives in `⚙️engine`.
//! LOSSLESS by construction: all 8 EPW header lines are retained (LOCATION fully typed + the
//! remaining 6 header blocks + DATA PERIODS structured) and every data record carries all 35
//! EPW columns (https://bigladdersoftware.com/epx/docs/9-6/auxiliary-programs/energyplus-weather-file-epw-data-dictionary.html) —
//! unlike energy's plugin-side `EpwWeather::parse` (`✏️s/🔌️plugins/🔋️energy/⚙️engine/site/🦀️component.rs`),
//! which reads a handful of columns with `unwrap_or(..)` silent defaults for its own derived
//! `WeatherRecord` view.
//!
//! 🔒 Every numeric-looking column (temperatures, radiation, LOCATION's lat/lon/elevation, …) is
//! stored as `String`, not `f64`/`u16`. This is a deliberate retention choice, not laziness: EPW
//! source text legally contains values like `20.0`/`1.0` (trailing `.0` kept), while Rust's `f64`
//! `Display` prints `20`/`1` (no trailing `.0`) — round-tripping through a numeric type would
//! silently mutate the byte-exact source text `codec_retention_law` requires. `year`/`month`/
//! `day`/`hour`/`minute` follow the same convention for uniformity (they're integer strings with
//! zero formatting ambiguity either way).

//#region 🔖️Location
/// 📍️ LOCATION header line — 9 fields + the `LOCATION` keyword itself = the spec's 10
/// comma-separated tokens (https://bigladdersoftware.com/epx/docs/9-6/auxiliary-programs/energyplus-weather-file-epw-data-dictionary.html#location).
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EpwLocation {
    pub city: String,
    pub state_province: String,
    pub country: String,
    pub source: String,
    pub wmo: String,
    pub latitude: String,
    pub longitude: String,
    pub time_zone: String,
    pub elevation: String,
}
//#endregion 🔖️Location

//#region 🔖️DataPeriods
/// 📅️ One named period from the DATA PERIODS header line.
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EpwDataPeriod {
    pub name: String,
    pub start_day_of_week: String,
    pub start_date: String,
    pub end_date: String,
}

/// 📅️ DATA PERIODS header line, structured: `records_per_hour` is a plain integer count (no
/// float-formatting hazard) plus a list of named periods. The leading period-count token is
/// derived from `periods.len()` on encode rather than stored (it is redundant, not lossy).
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EpwDataPeriods {
    pub records_per_hour: u32,
    pub periods: Vec<EpwDataPeriod>,
}
//#endregion 🔖️DataPeriods

//#region 🔖️Record
/// 🌡️ One hourly EPW data record — all 35 spec columns, in spec order, each a `String` (see
/// module doc comment for why). Field order here is the WIRE order used by `⚙️engine`'s
/// encoder/decoder and by this module's own diff/mutation index accessors below.
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EpwRecord {
    pub year: String,
    pub month: String,
    pub day: String,
    pub hour: String,
    pub minute: String,
    pub data_source_uncertainty: String,
    pub dry_bulb_temp: String,
    pub dew_point_temp: String,
    pub relative_humidity: String,
    pub atmospheric_pressure: String,
    pub extraterrestrial_horizontal_radiation: String,
    pub extraterrestrial_direct_normal_radiation: String,
    pub horizontal_infrared_radiation: String,
    pub global_horizontal_radiation: String,
    pub direct_normal_radiation: String,
    pub diffuse_horizontal_radiation: String,
    pub global_horizontal_illuminance: String,
    pub direct_normal_illuminance: String,
    pub diffuse_horizontal_illuminance: String,
    pub zenith_luminance: String,
    pub wind_direction: String,
    pub wind_speed: String,
    pub total_sky_cover: String,
    pub opaque_sky_cover: String,
    pub visibility: String,
    pub ceiling_height: String,
    pub present_weather_observation: String,
    pub present_weather_codes: String,
    pub precipitable_water: String,
    pub aerosol_optical_depth: String,
    pub snow_depth: String,
    pub days_since_last_snowfall: String,
    pub albedo: String,
    pub liquid_precip_depth: String,
    pub liquid_precip_quantity: String,
}

/// 🔢️ Canonical field count/order — shared by `⚙️engine` (wire codec), `🔺️diff` (index-keyed
/// sparse patch + hand-rolled diff codec) and `🧬️mutations` (`SetRecordField` variant).
pub const EPW_RECORD_FIELD_COUNT: usize = 35;

impl EpwRecord {
    /// 📤️ Reads one field by its canonical wire index (0-based, spec column order).
    pub fn field_at(&self, index: usize) -> Option<&str> {
        Some(match index {
            0 => &self.year,
            1 => &self.month,
            2 => &self.day,
            3 => &self.hour,
            4 => &self.minute,
            5 => &self.data_source_uncertainty,
            6 => &self.dry_bulb_temp,
            7 => &self.dew_point_temp,
            8 => &self.relative_humidity,
            9 => &self.atmospheric_pressure,
            10 => &self.extraterrestrial_horizontal_radiation,
            11 => &self.extraterrestrial_direct_normal_radiation,
            12 => &self.horizontal_infrared_radiation,
            13 => &self.global_horizontal_radiation,
            14 => &self.direct_normal_radiation,
            15 => &self.diffuse_horizontal_radiation,
            16 => &self.global_horizontal_illuminance,
            17 => &self.direct_normal_illuminance,
            18 => &self.diffuse_horizontal_illuminance,
            19 => &self.zenith_luminance,
            20 => &self.wind_direction,
            21 => &self.wind_speed,
            22 => &self.total_sky_cover,
            23 => &self.opaque_sky_cover,
            24 => &self.visibility,
            25 => &self.ceiling_height,
            26 => &self.present_weather_observation,
            27 => &self.present_weather_codes,
            28 => &self.precipitable_water,
            29 => &self.aerosol_optical_depth,
            30 => &self.snow_depth,
            31 => &self.days_since_last_snowfall,
            32 => &self.albedo,
            33 => &self.liquid_precip_depth,
            34 => &self.liquid_precip_quantity,
            _ => return None,
        })
    }

    /// 📥️ Writes one field by its canonical wire index. No-op if `index` is out of range.
    pub fn set_field_at(&mut self, index: usize, value: String) {
        match index {
            0 => self.year = value,
            1 => self.month = value,
            2 => self.day = value,
            3 => self.hour = value,
            4 => self.minute = value,
            5 => self.data_source_uncertainty = value,
            6 => self.dry_bulb_temp = value,
            7 => self.dew_point_temp = value,
            8 => self.relative_humidity = value,
            9 => self.atmospheric_pressure = value,
            10 => self.extraterrestrial_horizontal_radiation = value,
            11 => self.extraterrestrial_direct_normal_radiation = value,
            12 => self.horizontal_infrared_radiation = value,
            13 => self.global_horizontal_radiation = value,
            14 => self.direct_normal_radiation = value,
            15 => self.diffuse_horizontal_radiation = value,
            16 => self.global_horizontal_illuminance = value,
            17 => self.direct_normal_illuminance = value,
            18 => self.diffuse_horizontal_illuminance = value,
            19 => self.zenith_luminance = value,
            20 => self.wind_direction = value,
            21 => self.wind_speed = value,
            22 => self.total_sky_cover = value,
            23 => self.opaque_sky_cover = value,
            24 => self.visibility = value,
            25 => self.ceiling_height = value,
            26 => self.present_weather_observation = value,
            27 => self.present_weather_codes = value,
            28 => self.precipitable_water = value,
            29 => self.aerosol_optical_depth = value,
            30 => self.snow_depth = value,
            31 => self.days_since_last_snowfall = value,
            32 => self.albedo = value,
            33 => self.liquid_precip_depth = value,
            34 => self.liquid_precip_quantity = value,
            _ => {}
        }
    }

    /// 📤️ The 35 fields in wire order (spec column order).
    pub fn fields(&self) -> [&str; EPW_RECORD_FIELD_COUNT] {
        [
            &self.year, &self.month, &self.day, &self.hour, &self.minute, &self.data_source_uncertainty,
            &self.dry_bulb_temp, &self.dew_point_temp, &self.relative_humidity, &self.atmospheric_pressure,
            &self.extraterrestrial_horizontal_radiation, &self.extraterrestrial_direct_normal_radiation,
            &self.horizontal_infrared_radiation, &self.global_horizontal_radiation, &self.direct_normal_radiation,
            &self.diffuse_horizontal_radiation, &self.global_horizontal_illuminance, &self.direct_normal_illuminance,
            &self.diffuse_horizontal_illuminance, &self.zenith_luminance, &self.wind_direction, &self.wind_speed,
            &self.total_sky_cover, &self.opaque_sky_cover, &self.visibility, &self.ceiling_height,
            &self.present_weather_observation, &self.present_weather_codes, &self.precipitable_water,
            &self.aerosol_optical_depth, &self.snow_depth, &self.days_since_last_snowfall, &self.albedo,
            &self.liquid_precip_depth, &self.liquid_precip_quantity,
        ]
    }

    /// 📥️ Builds a record from exactly 35 wire-order fields.
    pub fn from_fields(f: [String; EPW_RECORD_FIELD_COUNT]) -> Self {
        let [year, month, day, hour, minute, data_source_uncertainty, dry_bulb_temp, dew_point_temp,
            relative_humidity, atmospheric_pressure, extraterrestrial_horizontal_radiation,
            extraterrestrial_direct_normal_radiation, horizontal_infrared_radiation, global_horizontal_radiation,
            direct_normal_radiation, diffuse_horizontal_radiation, global_horizontal_illuminance,
            direct_normal_illuminance, diffuse_horizontal_illuminance, zenith_luminance, wind_direction,
            wind_speed, total_sky_cover, opaque_sky_cover, visibility, ceiling_height,
            present_weather_observation, present_weather_codes, precipitable_water, aerosol_optical_depth,
            snow_depth, days_since_last_snowfall, albedo, liquid_precip_depth, liquid_precip_quantity] = f;
        Self {
            year, month, day, hour, minute, data_source_uncertainty, dry_bulb_temp, dew_point_temp,
            relative_humidity, atmospheric_pressure, extraterrestrial_horizontal_radiation,
            extraterrestrial_direct_normal_radiation, horizontal_infrared_radiation, global_horizontal_radiation,
            direct_normal_radiation, diffuse_horizontal_radiation, global_horizontal_illuminance,
            direct_normal_illuminance, diffuse_horizontal_illuminance, zenith_luminance, wind_direction,
            wind_speed, total_sky_cover, opaque_sky_cover, visibility, ceiling_height,
            present_weather_observation, present_weather_codes, precipitable_water, aerosol_optical_depth,
            snow_depth, days_since_last_snowfall, albedo, liquid_precip_depth, liquid_precip_quantity,
        }
    }
}
//#endregion 🔖️Record

use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Ids
pub const STDIO_EPW_DOCUMENT_SCHEMA: &str = "stdio.epw";
//#endregion 🔖️Ids

//#region 🔖️Snapshot
/// 📸️ Persisted `stdio.epw` snapshot — all 8 EPW header lines + every hourly record, lossless.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.epw")]
pub struct EpwSnapshot {
    #[state(artifact)]
    pub schema: String,
    /// 📍️ Header line 1 — fully typed (see [`EpwLocation`]).
    #[state(artifact)]
    pub location: EpwLocation,
    /// 🌡️ Header line 2, retained verbatim (design-day sizing summary; not structurally decoded).
    #[state(artifact)]
    #[serde(default)]
    pub design_conditions: String,
    /// 📆️ Header line 3, retained verbatim (named typical/extreme week ranges).
    #[state(artifact)]
    #[serde(default)]
    pub typical_extreme_periods: String,
    /// 🌍️ Header line 4, retained verbatim (per-depth monthly ground temperatures).
    #[state(artifact)]
    #[serde(default)]
    pub ground_temperatures: String,
    /// 🎉️ Header line 5, retained verbatim (holiday/DST flags).
    #[state(artifact)]
    #[serde(default)]
    pub holidays_dst: String,
    /// 💬️ Header line 6, retained verbatim.
    #[state(artifact)]
    #[serde(default)]
    pub comments_1: String,
    /// 💬️ Header line 7, retained verbatim.
    #[state(artifact)]
    #[serde(default)]
    pub comments_2: String,
    /// 📅️ Header line 8 — structured (see [`EpwDataPeriods`]).
    #[state(artifact)]
    pub data_periods: EpwDataPeriods,
    /// 🌡️ Hourly data records, in file order, each carrying all 35 EPW columns.
    #[state(artifact)]
    #[serde(default)]
    pub records: Vec<EpwRecord>,
}

impl Default for EpwSnapshot {
    fn default() -> Self {
        Self {
            schema: STDIO_EPW_DOCUMENT_SCHEMA.into(),
            location: EpwLocation::default(),
            design_conditions: String::new(),
            typical_extreme_periods: String::new(),
            ground_temperatures: String::new(),
            holidays_dst: String::new(),
            comments_1: String::new(),
            comments_2: String::new(),
            data_periods: EpwDataPeriods::default(),
            records: Vec::new(),
        }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedArtifactCodecs
// 🔗 Real EnergyPlus Weather File text codec lives in `🚪️io::decode_epw`/`encode_epw`
// (https://bigladdersoftware.com/epx/docs/9-6/auxiliary-programs/energyplus-weather-file-epw-data-dictionary.html).
impl store::ArtifactDsl for EpwSnapshot {
    const EXTENSION: &'static str = "epw";
    fn envelope_id() -> &'static str { STDIO_EPW_DOCUMENT_SCHEMA }

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        crate::artifacts::epw::standards::energyplus::subsets::any::io::decode_epw(body)
            .map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        let body = crate::artifacts::epw::standards::energyplus::subsets::any::io::encode_epw(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for EpwSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = crate::artifacts::epw::standards::energyplus::subsets::any::io::encode_epw(self).into_bytes();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        ).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes)
            .map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::ArtifactDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let _ = options;
        let text = String::from_utf8(inner).map_err(|e| store::PackError::Schema(e.to_string()))?;
        crate::artifacts::epw::standards::energyplus::subsets::any::io::decode_epw(&text).map_err(store::PackError::Schema)
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs
